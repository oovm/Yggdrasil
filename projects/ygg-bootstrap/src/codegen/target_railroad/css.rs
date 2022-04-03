pub const DEFAULT_CSS: &str = r#"
svg.railroad {
    background-color: hsl(30, 20%, 95%);
    background-size: 15px 15px;
    background-image: linear-gradient(to right, rgba(30, 30, 30, .05) 1px, transparent 1px),
                      linear-gradient(to bottom, rgba(30, 30, 30, .05) 1px, transparent 1px);
}

svg.railroad path {
    stroke-width: 3px;
    stroke: black;
    fill: transparent;
}

svg.railroad .debug {
    stroke-width: 1px;
    stroke: red;
}

svg.railroad text {
    font: 14px monospace;
    text-anchor: middle;
}

svg.railroad .nonterminal text {
    font-weight: bold;
}

svg.railroad text.comment {
    font: italic 12px;
}

svg.railroad rect {
    stroke-width: 3px;
    stroke: black;
}

svg.railroad g.labeledbox > rect {
    stroke-width: 1px;
    stroke: grey;
    stroke-dasharray: 5px;
    fill:rgba(90, 90, 150, .1);
}

svg.railroad .symbol > rect {
    fill:rgba(90, 90, 150, .1);
}
svg.railroad .regex > rect {
    fill: honeydew;
}
svg.railroad .string > rect {
    fill: cornsilk;
}
"#;