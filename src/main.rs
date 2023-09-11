use std::{env, fs, process::Command};

use defaults::default_false;
use serde::{Deserialize, Serialize};

mod defaults;

#[derive(Debug, Serialize, Deserialize)]
enum Outputs {
    STDOUT,
}

impl Default for Outputs {
    fn default() -> Self {
        Outputs::STDOUT
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Exec {
    exec: String,
    args: Option<Vec<String>>,
    #[serde(default)]
    output: Outputs,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sleep {
    time: usize,
}

#[derive(Deserialize, Serialize, Debug)]
struct Step {
    name: Option<String>,
    description: Option<String>,
    main: Exec,
    pre: Option<Exec>,
    post: Option<Exec>,
    #[serde(default)]
    retries: usize,
    timeout: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Pipeline {
    name: String,
    description: Option<String>,
    #[serde(default = "default_false")]
    use_env: bool,
    steps: Vec<Step>,
}

trait Executable {
    fn execute(&self);
}

impl Executable for Exec {
    fn execute(&self) {
        let mut command = Command::new(self.exec.clone());

        if self.args.as_ref().is_some() {
            command.args(self.args.as_ref().unwrap());
        }

        match self.output {
            Outputs::STDOUT => {
                let out = command.output().expect("can't get output");
                println!("\t\t{}", String::from_utf8(out.stdout).unwrap());
            }
        };
    }
}

impl Executable for Step {
    fn execute(&self) {
        if self.pre.as_ref().is_some() {
            self.pre.as_ref().unwrap().execute();
        }
        self.main.execute();

        if self.post.as_ref().is_some() {
            self.post.as_ref().unwrap().execute();
        }
    }
}

impl Executable for Pipeline {
    fn execute(&self) {
        println!("Pipeline: <{}> is started", self.name);
        for (i, step) in self.steps.iter().enumerate() {
            if step.name.is_none() {
                println!("\tStep [{}]", i + 1);
            } else {
                println!("\tStep [{}]", step.name.as_ref().unwrap().clone())
            }
            step.execute();
        }
    }
}

fn main() {
    let pipeline_file = env::args()
        .nth(1)
        .expect("provide path to the pipeline file");

    let contents =
        fs::read_to_string(pipeline_file).expect("Should have been able to read the file");

    let pipeline: Pipeline = serde_yaml::from_str(&contents).unwrap();

    pipeline.execute();

    // println!("{:?}", pipeline);
}
