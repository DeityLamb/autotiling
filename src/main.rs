use swayipc::{Connection, EventType, Fallible, WindowChange, WindowEvent};
use swayipc::{Event, NodeLayout, NodeType};

fn is_valid_event(event: &WindowEvent) -> bool {
    event.change == WindowChange::Focus && event.container.node_type != NodeType::FloatingCon
}

fn main() -> Fallible<()> {
    let mut conn = Connection::new()?;

    let events = Connection::new()?
        .subscribe([EventType::Window])?
        .filter_map(|event| match event {
            Ok(Event::Window(e)) if is_valid_event(&e) => Some(e.container),
            _ => None,
        })
        .filter(|node| node.rect.height < node.rect.width || NodeLayout::SplitV != node.layout)
        .map(|node| (node.rect.height, node.rect.width))
        .map(|(height, width)| if height > width { "splitv" } else { "splith" });

    for mode in events {
        conn.run_command(mode)?;
    }

    Ok(())
}
