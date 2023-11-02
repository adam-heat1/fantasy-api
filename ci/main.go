package main

import (
	"context"
	"fmt"
	"os"

	run "cloud.google.com/go/run/apiv2"
	runpb "cloud.google.com/go/run/apiv2/runpb"
	"dagger.io/dagger"
)

const GCR_SERVICE_URL = "projects/fantasy-app-403821/locations/us-central1/services/fantasy-api"
const GCR_PUBLISH_ADDRESS = "gcr.io/fantasy-app-403821/fantasy-api"

func main() {
	// create Dagger client
	ctx := context.Background()
	daggerClient, err := dagger.Connect(ctx, dagger.WithLogOutput(os.Stderr))
	if err != nil {
		panic(err)
	}
	defer daggerClient.Close()

	// get working directory on host
	source := daggerClient.Host().Directory(".", dagger.HostDirectoryOpts{
		Exclude: []string{"ci", "target"},
	})

	// build application
	rust := daggerClient.Container(dagger.ContainerOpts{Platform: "linux/amd64"}).
		From("rust:1.73")

	c := rust.
		WithDirectory("/fantasy-api", source).
		WithWorkdir("/fantasy-api").
		WithExec([]string{"cargo", "build", "--release"}).
		WithEntrypoint([]string{"cargo", "run"})

	// publish container to Google Container Registry
	addr, err := c.Publish(ctx, GCR_PUBLISH_ADDRESS)
	if err != nil {
		panic(err)
	}

	// print ref
	fmt.Println("Published at:", addr)

	// create Google Cloud Run client
	gcrClient, err := run.NewServicesClient(ctx)
	if err != nil {
		panic(err)
	}
	defer gcrClient.Close()

	// define service request
	gcrRequest := &runpb.UpdateServiceRequest{
		Service: &runpb.Service{
			Name:    GCR_SERVICE_URL,
			Ingress: runpb.IngressTraffic_INGRESS_TRAFFIC_ALL,
			Template: &runpb.RevisionTemplate{
				MaxInstanceRequestConcurrency: 1,
				Containers: []*runpb.Container{
					{
						Image: addr,
						Ports: []*runpb.ContainerPort{
							{
								Name:          "http1",
								ContainerPort: 8080,
							},
						},
					},
				},
				Scaling: &runpb.RevisionScaling{
					MinInstanceCount: 0,
					MaxInstanceCount: 1,
				},
			},
		},
	}

	// update service
	gcrOperation, err := gcrClient.UpdateService(ctx, gcrRequest)
	if err != nil {
		panic(err)
	}

	// wait for service request completion
	gcrResponse, err := gcrOperation.Wait(ctx)
	if err != nil {
		panic(err)
	}

	// print ref
	fmt.Println("Deployment for image", addr, "now available at", gcrResponse.Uri)

}
