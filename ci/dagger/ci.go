package dagger

import (
	"context"
	"fmt"
	"os"

	run "cloud.google.com/go/run/apiv2"
	runpb "cloud.google.com/go/run/apiv2/runpb"
	"dagger.io/dagger"
)

const PROJECT_ID = "fantasy-app-403821"
const APP_NAME = "fantasy-api"
const GCR_SERVICE_URL = "projects/" + PROJECT_ID + "/locations/us-central1/services/" + APP_NAME
const GAR_PUBLISH_ADDRESS = "us-central1-docker.pkg.dev/" + PROJECT_ID + "/" + APP_NAME + "/api"

func ci() {
	// create Dagger client
	ctx := context.Background()
	daggerClient, err := dagger.Connect(ctx, dagger.WithLogOutput(os.Stderr))
	if err != nil {
		panic(err)
	}
	defer daggerClient.Close()

	if os.Getenv("DATABASE_URL") == "" {
		panic("Environment variable DATABASE_URL is not set")
	}
	if os.Getenv("NTFY_UNKNOWN_MEDIA") == "" {
		panic("Environment variable NTFY_UNKNOWN_MEDIA is not set")
	}
	if os.Getenv("NTFY_UNKNOWN_ERROR") == "" {
		panic("Environment variable NTFY_UNKNOWN_ERROR is not set")
	}

	// get working directory on host
	source := daggerClient.Host().Directory(".", dagger.HostDirectoryOpts{
		Include: []string{"src", "Cargo.toml", "Cargo.lock"},
	})

	// build application
	rust := daggerClient.Container(dagger.ContainerOpts{Platform: "linux/amd64"}).
		From("rust:1.73")

	c := rust.
		WithDirectory("/fantasy-api", source).
		WithWorkdir("/fantasy-api").
		WithExec([]string{"cargo", "build", "--release"}).
		WithEnvVariable("DATABASE_URL", os.Getenv("DATABASE_URL")).
		WithEnvVariable("NTFY_UNKNOWN_MEDIA", os.Getenv("NTFY_UNKNOWN_MEDIA")).
		WithEnvVariable("NTFY_UNKNOWN_ERROR", os.Getenv("NTFY_UNKNOWN_ERROR")).
		WithEntrypoint([]string{"./target/release/fantasy-api"})

	// publish container to Google Container Registry
	addr, err := c.Publish(ctx, GAR_PUBLISH_ADDRESS)
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
						Resources: &runpb.ResourceRequirements{
							Limits: map[string]string{
								"cpu":    "1",
								"memory": "512Mi",
							},
						},
					},
				},
				Scaling: &runpb.RevisionScaling{
					MinInstanceCount: 0,
					MaxInstanceCount: 1,
				},
				ServiceAccount: "fantasy-api-gcr@fantasy-app-403821.iam.gserviceaccount.com",
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
