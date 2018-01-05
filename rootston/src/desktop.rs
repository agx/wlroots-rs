use wlroots::{Compositor, OutputBuilder, OutputBuilderResult, OutputDestruction,
              OutputManagerHandler};

use output::RootsOutput;
use view::RootsView;
use server::RootsServer;
use seat::RootsSeat;

pub struct RootsDesktop {
    views: Vec<RootsView>,
    outputs: Vec<RootsOutput>
}

impl RootsDesktop {
    pub fn new() -> RootsDesktop {
        wlr_log!(L_DEBUG, "Initializing roots desktop");
        RootsDesktop { views: Vec::new(),
                       outputs: Vec::new()
        }
    }
}

impl OutputManagerHandler for RootsDesktop {
    fn output_added<'output>(&mut self,
                             compositor: &mut Compositor,
                             builder: OutputBuilder<'output>)
                             -> Option<OutputBuilderResult<'output>> {
        let mut res = builder.build_best_mode(RootsOutput::new());
        let (w, h) = res.output.physical_dimensions();
        wlr_log!(L_DEBUG, "Output {} added", res.output.name());
        wlr_log!(L_DEBUG,
                 "{} {} {} x {}",
                 res.output.make(),
                 res.output.model(),
                 w,
                 h);
        // TODO Output config


        let state: &mut RootsServer = compositor.into();
        for seat in &mut state.seats {
            seat.configure_cursor(&mut res.output);
            seat.configure_xcursor(&mut res.output);
        }
        // NOTE NOTE NOTE possible solution:
        // https://play.rust-lang.org/?gist=7c37803505c71e9c3df37f21fd39fced&version=stable
        // mild boilerplate, but you will probably only have e.g one input and output handler,
        // so that's a plus.

        // NOTE: Supertraits do not work for this, since they would go the wrong way
        // (cast from e.g our trait to the wlroots-rs trait, which wouldn't work)

        // TODO FIXME
        // In rootston it configures the seats here for the output
        // so it needs access to the input manager...
        // in the examples we just stored that in the global data and we can do
        // that there, but not here (because we need more info than the cursors, we need
        // all of the input and that's just something we return in these constructors)
        // But I wonder if there's a safe way we can expose the concrete
        // input manager here (e.g the underlying concrete type)
        //
        // the compositor owns it, but it's not exposed publically so even though we
        // have a reference up to it, we can't modify it. That's probably for the best,
        // because it's a Box<InputManager>, which still just has a box of the trait.
        // so unless there's a way to upcast to the concrete implementation...that's not really
        // a solution (and again, that exposes a lot more than I'm comfortable with, so it
        // it could very, very easily lead to memory unsafety)
        Some(res)
    }

    fn output_removed(&mut self, compositor: &mut Compositor, output: OutputDestruction) {
        // TODO
    }
}
