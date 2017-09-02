use timetype::TimeType;

error_chain! {
    types {
        KairosError, KairosErrorKind, ResultExt, Result;
    }

    links {
    }

    foreign_links {
    }

    errors {

        UnknownError {
            description("Unknown Error")
            display("Unknown Error")
        }

        CannotAdd(a: TimeType, b: TimeType) {
            description("Cannot add")
            display("Cannot add: {:?} + {:?}", a, b)
        }

        CannotSub(a: TimeType, b: TimeType) {
            description("Cannot subtract")
            display("Cannot subtract: {:?} - {:?}", a, b)
        }

    }

}
