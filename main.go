package main

import (
	"fmt"

	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
	"github.com/pulumi/pulumi-gcp/sdk/v7/go/gcp/cloudrun"
	"github.com/pulumi/pulumi-gcp/sdk/v7/go/gcp/sql"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		cfg := config.New(ctx, "")
		projectID := cfg.Get("projectId")
		if projectID == "" {
			panic("Missing required config `projectId`")
		}
		region := cfg.Get("region")
		if region == "" {
			panic("Missing required config `region`")
		}
		appName := cfg.Get("appName")
		if appName == "" {
			panic("Missing required config `appName`")
		}
		databaseName := cfg.Get("databaseName")
		if databaseName == "" {
			panic("Missing required config `databaseName`")
		}

		databaseUrl := cfg.Get("databaseUrl")
		if databaseUrl == "" {
			panic("Missing required config `databaseUrl`")
		}

		ntfyUnknownMedia := cfg.Get("ntfyUnknownMedia")
		if ntfyUnknownMedia == "" {
			panic("Missing required config `ntfyUnknownMedia`")
		}

		ntfyUnknownError := cfg.Get("ntfyUnknownError")
		if ntfyUnknownError == "" {
			panic("Missing required config `ntfyUnknownError`")
		}

		apiDomain := cfg.Get("apiDomain")
		if apiDomain == "" {
			panic("Missing required config `apiDomain`")
		}

		appImage, err := docker.NewImage(ctx, appName, &docker.ImageArgs{
			ImageName: pulumi.Sprintf("%s-docker.pkg.dev/%s/%s/api", region, projectID, appName),
			Build: &docker.DockerBuildArgs{
				Context:    pulumi.String("./"),
				Dockerfile: pulumi.String("./Dockerfile"),
				Platform:   pulumi.String("linux/amd64"),
			}})
		if err != nil {
			return err
		}

		fmt.Println(appImage.ImageName.ToStringOutput())

		database, err := sql.NewDatabaseInstance(ctx, databaseName, &sql.DatabaseInstanceArgs{
			Region:             pulumi.String(region),
			DatabaseVersion:    pulumi.String("POSTGRES_15"),
			DeletionProtection: pulumi.Bool(true),
			Name:               pulumi.String(databaseName),
			Project:            pulumi.String(projectID),
			Settings: &sql.DatabaseInstanceSettingsArgs{
				Tier:                      pulumi.String("db-f1-micro"),
				DeletionProtectionEnabled: pulumi.Bool(false),
				BackupConfiguration: &sql.DatabaseInstanceSettingsBackupConfigurationArgs{
					BackupRetentionSettings: &sql.DatabaseInstanceSettingsBackupConfigurationBackupRetentionSettingsArgs{
						RetainedBackups: pulumi.Int(7),
					},
					Enabled: pulumi.Bool(true),
				},
				IpConfiguration: &sql.DatabaseInstanceSettingsIpConfigurationArgs{
					AuthorizedNetworks: &sql.DatabaseInstanceSettingsIpConfigurationAuthorizedNetworkArray{
						&sql.DatabaseInstanceSettingsIpConfigurationAuthorizedNetworkArgs{
							Value: pulumi.String("136.32.221.141"),
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}

		// Deploy the Docker image to Cloud Run
		service, err := cloudrun.NewService(ctx, appName, &cloudrun.ServiceArgs{
			Location: pulumi.String(region),
			Project:  pulumi.String(projectID),
			Name:     pulumi.String(appName),
			Template: &cloudrun.ServiceTemplateArgs{
				Spec: &cloudrun.ServiceTemplateSpecArgs{
					Containers: cloudrun.ServiceTemplateSpecContainerArray{
						&cloudrun.ServiceTemplateSpecContainerArgs{
							Image: appImage.RepoDigest,
							Ports: cloudrun.ServiceTemplateSpecContainerPortArray{
								&cloudrun.ServiceTemplateSpecContainerPortArgs{
									Name:          pulumi.String("http1"),
									ContainerPort: pulumi.Int(8080),
								},
							},
							Resources: &cloudrun.ServiceTemplateSpecContainerResourcesArgs{
								Limits: pulumi.StringMap(map[string]pulumi.StringInput{
									"cpu":    pulumi.String("1"),
									"memory": pulumi.String("512Mi"),
								}),
							},
							Envs: cloudrun.ServiceTemplateSpecContainerEnvArray{
								&cloudrun.ServiceTemplateSpecContainerEnvArgs{
									Name:  pulumi.String("DATABASE_URL"),
									Value: pulumi.String(databaseUrl),
								},
								&cloudrun.ServiceTemplateSpecContainerEnvArgs{
									Name:  pulumi.String("NTFY_UNKNOWN_MEDIA"),
									Value: pulumi.String(ntfyUnknownMedia),
								},
								&cloudrun.ServiceTemplateSpecContainerEnvArgs{
									Name:  pulumi.String("NTFY_UNKNOWN_ERROR"),
									Value: pulumi.String(ntfyUnknownError),
								},
							},
						},
					},
					ContainerConcurrency: pulumi.Int(5),
				},
				Metadata: &cloudrun.ServiceTemplateMetadataArgs{
					Annotations: pulumi.StringMap{
						"run.googleapis.com/cloudsql-instances": database.ConnectionName,
					},
				},
			},
			Traffics: cloudrun.ServiceTrafficArray{
				&cloudrun.ServiceTrafficArgs{
					Percent:        pulumi.Int(100),
					LatestRevision: pulumi.Bool(true),
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{appImage}))
		if err != nil {
			return err
		}

		_, err = cloudrun.NewIamMember(ctx, fmt.Sprintf("%s-iam", appName), &cloudrun.IamMemberArgs{
			Service:  service.Name,
			Location: service.Location,
			Role:     pulumi.String("roles/run.invoker"),
			Member:   pulumi.String("allUsers"),
		})
		if err != nil {
			return err
		}

		_, err = cloudrun.NewDomainMapping(ctx, "heat1-api-domain-mapping", &cloudrun.DomainMappingArgs{
			Location: pulumi.String(region),
			Name:     pulumi.String(apiDomain),
			Metadata: &cloudrun.DomainMappingMetadataArgs{
				Namespace: pulumi.String(projectID),
			},
			Spec: &cloudrun.DomainMappingSpecArgs{
				RouteName: service.Name,
			},
		})
		if err != nil {
			return err
		}

		// Export the URL of the Cloud Run service
		ctx.Export("url", service.Statuses.Index(pulumi.Int(0)).Url())
		return nil
	})
}
