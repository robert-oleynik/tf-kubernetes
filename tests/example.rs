use tf_bindgen::{app::App, stack::Stack};
use tf_kubernetes::kubernetes::resource::kubernetes_pod::*;

#[test]
fn nginx() {
    let app = App::default();
    let stack = Stack::new(&app, "nginx");

    let meta = KubernetesPodMetadata::builder().name("nginx").build();
    let port = KubernetesPodSpecContainerPort::builder()
        .container_port(80_usize)
        .build();
    let container = KubernetesPodSpecContainer::builder()
        .name("nginx")
        .image("nginx")
        .port(vec![Some(port)])
        .build();
    let spec = KubernetesPodSpec::builder()
        .container(vec![Some(container)])
        .build();

    KubernetesPod::create(&stack, "nginx")
        .metadata(meta)
        .spec(spec)
        .build();
}
