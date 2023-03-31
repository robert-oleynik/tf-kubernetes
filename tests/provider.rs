use tf_bindgen::cli::Terraform;
use tf_bindgen::Stack;
use tf_kubernetes::resource::kubernetes_namespace::*;
use tf_kubernetes::Kubernetes;

#[test]
fn default_provider() {
    let stack = Stack::new("default_provider");

    Kubernetes::create(&stack).build();

    KubernetesNamespace::create(&stack, "default")
        .metadata(KubernetesNamespaceMetadata::builder().build())
        .build();

    if !Terraform::init(&stack)
        .unwrap()
        .output()
        .unwrap()
        .status
        .success()
    {
        panic!()
    }
    if !Terraform::validate(&stack)
        .unwrap()
        .output()
        .unwrap()
        .status
        .success()
    {
        panic!()
    }
}

#[test]
fn config_path_provider() {
    let stack = Stack::new("config_path_provider");

    Kubernetes::create(&stack)
        .config_path("~/.kube/config")
        .build();

    KubernetesNamespace::create(&stack, "default")
        .metadata(KubernetesNamespaceMetadata::builder().build())
        .build();

    if !Terraform::init(&stack)
        .unwrap()
        .output()
        .unwrap()
        .status
        .success()
    {
        panic!()
    }
    if !Terraform::validate(&stack)
        .unwrap()
        .output()
        .unwrap()
        .status
        .success()
    {
        panic!()
    }
}
