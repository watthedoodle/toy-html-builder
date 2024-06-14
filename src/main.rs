fn main() {
    let html = Html::new().add(Node {
        text: Html::new()
            .add(Node {
                text: "coffee".to_string(),
                tag: "li".to_string(),
            })
            .add(Node {
                text: "tea".to_string(),
                tag: "li".to_string(),
            })
            .add(Node {
                text: "milk".to_string(),
                tag: "li".to_string(),
            })
            .add(Node {
                text: "water".to_string(),
                tag: "li".to_string(),
            }),
        tag: "ul".to_string(),
    });
    print!("{}", html.to_text());
}

trait ToText {
    fn to_text(&self) -> String;
}

impl ToText for String {
    fn to_text(&self) -> String {
        self.to_string()
    }
}

impl<T: ToText> ToText for Html<T> {
    fn to_text(&self) -> String {
        let mut html = String::new();
        for e in &self.items {
            if e.tag == "" {
                html.push_str(&e.text.to_text());
            } else {
                html.push_str(&format!(
                    "<{tag}>{text}</{tag}>",
                    tag = e.tag,
                    text = e.text.to_text()
                ));
            }
        }
        html
    }
}

struct Html<T: ToText> {
    items: Vec<Node<T>>,
}

struct Node<T: ToText> {
    text: T,
    tag: String,
}

impl<T: ToText> Html<T> {
    pub fn new() -> Self {
        Html { items: Vec::new() }
    }

    fn add(mut self, node: Node<T>) -> Self {
        self.items.push(node);
        self
    }
}
