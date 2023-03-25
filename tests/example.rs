use tf_bindgen::app::App;
use tf_bindgen::stack::Stack;
use tf_kubernetes::kubernetes::resource::kubernetes_namespace::*;
use tf_kubernetes::kubernetes::resource::kubernetes_pod::*;
use tf_kubernetes::kubernetes::Kubernetes;

#[test]
fn nginx() {
    let app = App::default();
    let stack = Stack::new(&app, "nginx");

    Kubernetes::create(&stack).build();

    let meta = KubernetesPodMetadata::builder().name("nginx").build();
    let port = KubernetesPodSpecContainerPort::builder()
        .container_port(80)
        .build();
    let container = KubernetesPodSpecContainer::builder()
        .name("nginx")
        .image("nginx")
        .port(vec![port])
        .build();
    let spec = KubernetesPodSpec::builder()
        .container(vec![container])
        .build();

    KubernetesPod::create(&stack, "nginx")
        .metadata(meta)
        .spec(spec)
        .build();

    app.validate(true).unwrap()
}

#[test]
fn nginx_link() {
    let app = App::default();
    let stack = Stack::new(&app, "nginx-link");

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
        .port(vec![port])
        .build();
    let spec = KubernetesPodSpec::builder()
        .container(vec![container])
        .build();

    KubernetesPod::create(&stack, "nginx")
        .metadata(meta)
        .spec(spec)
        .build();

    app.validate(true).unwrap()
}

#[test]
fn nginx_macro() {
    let app = App::default();
    let stack = Stack::new(&app, "nginx-macro");

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

    app.validate(true).unwrap()
}
