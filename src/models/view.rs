use crate::models::types::{ExtractorCard, FlowStep, Layer, ResponseType, RouteRow};

fn color_to_text(color: &str) -> &'static str {
    match color {
        "coral" => "text-rose-400",
        "blue" => "text-blue-400",
        "amber" => "text-amber-400",
        "teal" => "text-teal-400",
        "green" => "text-emerald-400",
        _ => "text-violet-400",
    }
}

fn color_to_card(color: &str) -> &'static str {
    match color {
        "coral" => "bg-rose-500/10 border-rose-500/20",
        "blue" => "bg-blue-500/10 border-blue-500/20",
        "amber" => "bg-amber-500/10 border-amber-500/20",
        "teal" => "bg-teal-500/10 border-teal-500/20",
        _ => "bg-violet-500/10 border-violet-500/20",
    }
}

fn color_to_badge(color: &str) -> &'static str {
    match color {
        "coral" => "bg-rose-500/10 border-rose-500/20 text-rose-400",
        "blue" => "bg-blue-500/10 border-blue-500/20 text-blue-400",
        "amber" => "bg-amber-500/10 border-amber-500/20 text-amber-400",
        "teal" => "bg-teal-500/10 border-teal-500/20 text-teal-400",
        _ => "bg-violet-500/10 border-violet-500/20 text-violet-400",
    }
}

fn http_method_to_tag_cls(method: &str) -> &'static str {
    match method {
        "GET" => "bg-emerald-500/10 text-emerald-400",
        "POST" => "bg-amber-500/10 text-amber-400",
        "DELETE" => "bg-red-500/10 text-red-400",
        _ => "bg-blue-500/10 text-blue-400",
    }
}

fn layer_to_tag_cls(layer: &str) -> &'static str {
    match layer {
        "tokio" => "bg-violet-500/10 border-violet-500/20 text-violet-400",
        "hyper" => "bg-teal-500/10 border-teal-500/20 text-teal-400",
        "tower" => "bg-amber-500/10 border-amber-500/20 text-amber-400",
        "axum" => "bg-blue-500/10 border-blue-500/20 text-blue-400",
        _ => "bg-rose-500/10 border-rose-500/20 text-rose-400",
    }
}
fn color_to_band(color: &str) -> &'static str {
    match color {
        "coral" => "bg-rose-500/40 group-hover:bg-rose-500",
        "blue" => "bg-blue-500/40 group-hover:bg-blue-500",
        "amber" => "bg-amber-500/40 group-hover:bg-amber-500",
        "teal" => "bg-teal-500/40 group-hover:bg-teal-500",
        _ => "bg-violet-500/40 group-hover:bg-violet-500",
    }
}

fn color_to_button_hover(color: &str) -> &'static str {
    match color {
        "coral" => "hover:border-rose-500/30 hover:bg-rose-500/5",
        "blue" => "hover:border-blue-500/30 hover:bg-blue-500/5",
        "amber" => "hover:border-amber-500/30 hover:bg-amber-500/5",
        "teal" => "hover:border-teal-500/30 hover:bg-teal-500/5",
        _ => "hover:border-violet-500/30 hover:bg-violet-500/5",
    }
}

// ── TailwindColored trait ─────────────────────────────────────────────────────
// Any type that exposes a semantic colour token gets the CSS helpers for free.

pub trait TailwindColored {
    fn color(&self) -> &str;

    fn text_cls(&self) -> &'static str {
        color_to_text(self.color())
    }
    fn card_cls(&self) -> &'static str {
        color_to_card(self.color())
    }
    fn badge_cls(&self) -> &'static str {
        color_to_badge(self.color())
    }

    fn icon_cls(&self) -> &'static str {
        color_to_card(self.color())
    }
    fn band_cls(&self) -> &'static str {
        color_to_band(self.color())
    }
    fn button_hover_cls(&self) -> &'static str {
        color_to_button_hover(self.color())
    }
}

// ── Trait implementations ─────────────────────────────────────────────────────

impl TailwindColored for Layer {
    fn color(&self) -> &str {
        self.color_class
    }
}

impl TailwindColored for ExtractorCard {
    // also needs icon_cls which is just color_to_card — covered by card_cls()
    fn color(&self) -> &str {
        self.color
    }
}

impl TailwindColored for ResponseType {
    fn color(&self) -> &str {
        self.color
    }
}

// ── FlowStep — uses layer names, not colour tokens ────────────────────────────
// Does not implement TailwindColored; gets its own impl.

impl FlowStep {
    pub fn tag_cls(&self) -> &'static str {
        layer_to_tag_cls(self.layer)
    }
}

impl RouteRow {
    pub fn tag_cls(&self) -> &'static str {
        http_method_to_tag_cls(self.method)
    }
}
