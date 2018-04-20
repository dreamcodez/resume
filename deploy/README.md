
### AWS Codebuild must have these env vars:
```
CONTAINER_REPO=<ecr repo path>
HELM_CHART_RELEASE=<release name in cluster (upgrade only script for now)>
SYSADMIN_PHONE=<phone number of person to be notified about deployments etc>
```

### Open-source base build image for aws codebuild, for deploying to helm/kubernetes environments w/ credstash
 - https://github.com/dreamcodez/helm-deploykit
 - must used privileged docker flag
