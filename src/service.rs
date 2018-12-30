use super::{lookup, Options, Reader};

use futures::future;
use hyper::rt::Future;
use hyper::service::{MakeService, Service};
use hyper::{Body, Request, Response};

use std::sync::Arc;

pub struct LookupService {
    db: Arc<Reader>,
    opts: Arc<Options>,
}

impl Service for LookupService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        Box::new(future::ok(lookup(req, &self.db, &self.opts)))
    }
}

pub struct MakeLookupService {
    db: Arc<Reader>,
    opts: Arc<Options>,
}

impl MakeService<LookupService> for MakeLookupService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Service = LookupService;
    type Future = Box<Future<Item = Self::Service, Error = Self::MakeError> + Send>;
    type MakeError = hyper::Error;

    fn make_service(&mut self, _ctx: LookupService) -> Self::Future {
        let svc = LookupService {
            db: Arc::clone(&self.db),
            opts: Arc::clone(&self.opts),
        };
        Box::new(future::ok(svc))
    }
}

impl MakeLookupService {
    // we want to take ownership here so main doesnt have to deal with ref counting etc for scope
    pub fn new(db: Reader, opts: Options) -> MakeLookupService {
        MakeLookupService {
            db: Arc::new(db),
            opts: Arc::new(opts),
        }
    }
}
