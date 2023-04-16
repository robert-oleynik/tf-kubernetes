use tf_bindgen::cli::Terraform;
use tf_bindgen::Stack;
use tf_kubernetes::resource::kubernetes_namespace::*;
use tf_kubernetes::resource::kubernetes_pod::{self, *};
use tf_kubernetes::Kubernetes;

#[test]
fn nginx() {
    let stack = Stack::new("nginx");

    Kubernetes::create(&stack).build();

    let meta = KubernetesPodMetadata::builder().name("nginx").build();
    let port = KubernetesPodSpecContainerPort::builder()
        .container_port(80)
        .build();
    let container = KubernetesPodSpecContainer::builder()
        .name("nginx")
        .image("nginx")
        .port(port)
        .build();
    let spec = KubernetesPodSpec::builder().container(container).build();

    KubernetesPod::create(&stack, "nginx")
        .metadata(meta)
        .spec(spec)
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
fn nginx_link() {
    let stack = Stack::new("nginx-link");

    Kubernetes::create(&stack).build();

    let meta = KubernetesNamespaceMetadata::builder().name("nginx").build();
    let ns = KubernetesNamespace::create(&stack, "nginx")
        .metadata(meta)
        .build();

    let meta = KubernetesPodMetadata::builder()
        .namespace(&ns.metadata[0].name)
        .name("nginx")
        .build();
    let port = KubernetesPodSpecContainerPort::builder()
        .container_port(80)
        .build();
    let container = KubernetesPodSpecContainer::builder()
        .name("nginx")
        .image("nginx")
        .port(port)
        .build();
    let spec = KubernetesPodSpec::builder().container(container).build();

    KubernetesPod::create(&stack, "nginx")
        .metadata(meta)
        .spec(spec)
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
fn nginx_macro() {
    let stack = Stack::new("nginx-macro");

    Kubernetes::create(&stack).build();

    tf_bindgen::codegen::resource! {
        &stack,
        resource "kubernetes_pod" "nginx" {
            metadata {
                name = "nginx"
            }
            spec {
                container {
                    name = "nginx"
                    image = "nginx"

                    port {
                        container_port = 80
                    }
                }
            }
        }
    };

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
