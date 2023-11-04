package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"dagger.io/dagger"
	"google.golang.org/api/compute/v1"
)

const GCR_SERVICE_URL = "projects/fantasy-app-403821/locations/us-central1/services/fantasy-api"
const GCR_PUBLISH_ADDRESS = "gcr.io/fantasy-app-403821/fantasy-api"

// us-central1-docker.pkg.dev/fantasy-app-403821/fantasy-api

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
		Include: []string{"src", "Cargo.toml", "Cargo.lock"},
	})

	// build application
	rust := daggerClient.Container(dagger.ContainerOpts{Platform: "linux/amd64"}).
		From("rust:1.73")

	c := rust.
		WithDirectory("/fantasy-api", source).
		// WithExec([]string{"USER=root", "cargo", "new", "--bin", "fantasy-api"}).
		WithWorkdir("/fantasy-api").
		WithExec([]string{"cargo", "build", "--release"}).
		// WithExec([]string{"rm", "src/*.rs"}).
		// WithExec([]string{"rm", "./target/release/deps/fantasy_api*"}).
		// WithExec([]string{"cargo", "install", "--path", "."}).
		WithEntrypoint([]string{"./target/release/fantasy-api"})

	// publish container to Google Container Registry
	addr, err := c.Publish(ctx, GCR_PUBLISH_ADDRESS)
	if err != nil {
		panic(err)
	}

	// print ref
	fmt.Println("Published at:", addr)

	computeService, err := compute.NewService(ctx)
	if err != nil {
		log.Fatalf("Failed to create Compute Engine service client: %v", err)
	}

	instanceName := "fantasy-api"
	metadataValue := "spec:\n  containers:\n    - name: fantasy-api\n      image: " + GCR_PUBLISH_ADDRESS + "\n      stdin: false\n      tty: false,\n  restartPolicy: Always\n  port: 8080\n"

	updatedInstance := &compute.Instance{
		Name:        instanceName,
		Description: "API to host the fantasy app",
		MachineType: "e2-micro",

		// NetworkInterfaces: []*compute.NetworkInterface{
		// 	{
		// 		Network: "global/networks/default",
		// 		AccessConfigs: []*compute.AccessConfig{
		// 			{
		// 				Name: "External NAT",
		// 				Type: "ONE_TO_ONE_NAT",
		// 			},
		// 		},
		// 	},
		// },
		Metadata: &compute.Metadata{
			Items: []*compute.MetadataItems{
				{
					Key:   "gce-container-declaration",
					Value: &metadataValue,
				},
			},
		},
	}

	// define service request
	// gcrRequest := &compute.UpdateServiceRequest{
	// 	Service: &runpb.Service{
	// 		Name:    GCR_SERVICE_URL,
	// 		Ingress: runpb.IngressTraffic_INGRESS_TRAFFIC_ALL,
	// 		Template: &runpb.RevisionTemplate{
	// 			MaxInstanceRequestConcurrency: 1,
	// 			Containers: []*runpb.Container{
	// 				{
	// 					Image: addr,
	// 					Ports: []*runpb.ContainerPort{
	// 						{
	// 							Name:          "http1",
	// 							ContainerPort: 8080,
	// 						},
	// 					},
	// 					Resources: &runpb.ResourceRequirements{
	// 						Limits: map[string]string{
	// 							"cpu":    "1",
	// 							"memory": "512Mi",
	// 						},
	// 					},
	// 				},
	// 			},
	// 			Scaling: &runpb.RevisionScaling{
	// 				MinInstanceCount: 0,
	// 				MaxInstanceCount: 1,
	// 			},
	// 		},
	// 	},
	// }

	// Update the GCE instance.
	op, err := computeService.Instances.Update("fantasy-app-403821", "us-cental1-c", instanceName, updatedInstance).Context(ctx).Do()
	if err != nil {
		log.Fatalf("Failed to update GCE instance: %v", err)
	}

	fmt.Printf("Instance update operation: %v\n", op)

	// // wait for service request completion
	// gcrResponse, err := gcrOperation.Wait(ctx)
	// if err != nil {
	// 	panic(err)
	// }

	// print ref
	// fmt.Println("Deployment for image", addr, "now available at", gcrResponse.Uri)

}
