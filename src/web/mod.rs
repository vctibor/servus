use handlebars::Handlebars;
use std::sync::Arc;
use serde::Serialize;
use serde_json::json;
use warp::Filter;



const SIDEPANEL: &str = "_sidepanel";
const JOBS: &str = "jobs";
const MACHINES: &str = "machines";
const USERS: &str = "users";
const LOG: &str = "log";

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
    where
        T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

pub struct Web {
}




impl Web {

    pub fn new() -> Web {
        Web {  }
    }

    pub async fn serve(self: &Web) {

        /*
        let template_sidepanel = include_str!("../../templates/_sidepanel.hbs");
        let template_jobs = include_str!("../../templates/jobs.hbs");
        let template_machines = include_str!("../../templates/machines.hbs");
        let template_users = include_str!("../../templates/users.hbs");
        let template_log = include_str!("../../templates/log.hbs");

        let mut hb = Handlebars::new();

        hb.register_template_string(SIDEPANEL, template_sidepanel).unwrap();
        hb.register_template_string(JOBS, template_jobs).unwrap();
        hb.register_template_string(MACHINES, template_machines).unwrap();
        hb.register_template_string(USERS, template_users).unwrap();
        hb.register_template_string(LOG, template_log).unwrap();

        // Turn Handlebars instance into a Filter so we can combine it easily with others...
        let hb = Arc::new(hb);

        let handlebars = move |with_template| render(with_template, hb.clone());

        let connection = establish_connection();

        let static_files_route = warp::path("static").and(warp::fs::dir("static"));

        /*
        let jobs_route = warp::path(JOBS)
            .and(warp::path::end())
            .map(|| WithTemplate {
                name: JOBS,
                value: json!({
                    "jobs" : true
                }),
            })
            .map(handlebars.clone());

        let machines_route = warp::path(MACHINES)
            .and(warp::path::end())
            .map(|| WithTemplate {
                name: MACHINES,
                value: json!({
                    "machines" : true
                }),
            })
            .map(handlebars.clone());

        let users_route = warp::path(USERS)
            .and(warp::path::end())
            .map(|| WithTemplate {
                name: USERS,
                value: json!({
                    "users" : true
                }),
            })
            .map(handlebars.clone());

        let log_route = warp::path(LOG)
            .and(warp::path::end())
            .map(|| WithTemplate {
                name: LOG,
                value: json!({
                    "log" : true
                }),
            })
            .map(handlebars.clone());
        */

        let get_users_route = warp::get()
            .and(warp::path("get_users"))
            .and(warp::body::json())
            .map(|_|
                warp::reply::json(&get_users(&connection))
            );

        let routes = static_files_route
            /*
            .or(jobs_route)
            .or(machines_route)
            .or(users_route)
            .or(log_route)
            */
            .or(get_users_route);

        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
        */
    }
}

