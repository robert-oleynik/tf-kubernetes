use tf_bindgen::{app::App, stack::Stack};
use tf_kubernetes::kubernetes::resource::kubernetes_pod::*;

#[test]
fn nginx() {
    let app = App::default();
    let stack = Stack::new(&app, "nginx");

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
}
