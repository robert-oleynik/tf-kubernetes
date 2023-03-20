use tf_bindgen::{app::App, stack::Stack};
use tf_kubernetes::kubernetes::resource::kubernetes_pod::{self, *};

#[test]
fn nginx() {
    let app = App::default();
    let stack = Stack::new(&app, "nginx");

    let meta = kubernetes_pod::Metadata::builder().name("nginx").build();
    let port = kubernetes_pod::SpecContainerPort::builder()
        .container_port(80)
        .build();
    let container = kubernetes_pod::SpecContainer::builder()
        .name("nginx")
        .image("nginx")
        .port(vec![port])
        .build();
    let spec = kubernetes_pod::Spec::builder()
        .container(vec![container])
        .build();

    KubernetesPod::create(&stack, "nginx")
        .metadata(meta)
        .spec(spec)
        .build();
}
