// Simple element to understand render stages: request_layout, prepaint and paint

use gpui::*;
use smallvec::SmallVec;

/// A frame state for a `SizeObserver` element, which contains layout IDs for its children.
///
/// This struct is used internally by the `SizeObserver` element to manage the layout state of its children
/// during the UI update cycle. It holds a small vector of `LayoutId` values, each corresponding to
/// a child element of the `SizeObserver`. These IDs are used to query the layout engine for the computed
/// bounds of the children after the layout phase is complete.
pub struct SizeObserverFrameState {
    child_layout_ids: SmallVec<[LayoutId; 2]>,
}

pub(crate) type MeasureSizeListener = Box<dyn Fn(Size<Pixels>, &mut WindowContext) + 'static>;

pub struct SizeObserver {
    interactivity: Interactivity,
    children: SmallVec<[AnyElement; 2]>,
    size_callback: Option<MeasureSizeListener>,
}

// create new SizeObserver element
pub fn size_observer() -> SizeObserver {
    SizeObserver {
        interactivity: Interactivity::default(),
        children: SmallVec::default(),
        size_callback: None,
    }
}

impl SizeObserver {
    pub fn on_size_measured(
        mut self,
        callback: impl Fn(Size<Pixels>, &mut WindowContext) + 'static,
    ) -> Self {
        self.size_callback = Some(Box::new(callback));

        self
    }
}

impl Element for SizeObserver {
    type RequestLayoutState = SizeObserverFrameState;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<crate::ElementId> {
        self.interactivity.element_id.clone()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut child_layout_ids = SmallVec::new();
        let layout_id = self
            .interactivity
            .request_layout(global_id, cx, |style, cx| {
                cx.with_text_style(style.text_style().cloned(), |cx| {
                    child_layout_ids = self
                        .children
                        .iter_mut()
                        .map(|child| child.request_layout(cx))
                        .collect::<SmallVec<_>>();
                    cx.request_layout(style, child_layout_ids.iter().copied())
                })
            });

        (layout_id, SizeObserverFrameState { child_layout_ids })
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Option<Hitbox> {
        let mut child_min = point(Pixels::MAX, Pixels::MAX);
        let mut child_max = Point::default();

        let content_size = if request_layout.child_layout_ids.is_empty() {
            bounds.size
        } else {
            for child_layout_id in &request_layout.child_layout_ids {
                let child_bounds = cx.layout_bounds(*child_layout_id);
                child_min = child_min.min(&child_bounds.origin);
                child_max = child_max.max(&child_bounds.bottom_right());
            }
            (child_max - child_min).into()
        };

        if let Some(callback) = &mut self.size_callback {
            callback(content_size, cx);
        }

        self.interactivity.prepaint(
            global_id,
            bounds,
            content_size,
            cx,
            |_style, scroll_offset, hitbox, cx| {
                cx.with_element_offset(scroll_offset, |cx| {
                    for child in &mut self.children {
                        child.prepaint(cx);
                    }
                });
                hitbox
            },
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Option<Hitbox>,
        cx: &mut WindowContext,
    ) {
        self.interactivity
            .paint(global_id, bounds, hitbox.as_ref(), cx, |_style, cx| {
                for child in &mut self.children {
                    child.paint(cx);
                }
            });
    }
}

impl IntoElement for SizeObserver {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for SizeObserver {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for SizeObserver {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl ParentElement for SizeObserver {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
