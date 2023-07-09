use dialoguer::{Input, Select};
use std::fs::{create_dir, File};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let (upper_kind, lower_kind, hyphenated_name, capitalized_name) = input()?;
    let root = root_dir()?;

    let dir = format!("{root}/src/components/{lower_kind}/{hyphenated_name}");

    create_dir(&dir)?;
    write(format!("{dir}/{capitalized_name}.tsx"), tsx(&capitalized_name))?;
    write(format!("{dir}/{capitalized_name}.module.scss"), scss())?;
    write(format!("{dir}/{capitalized_name}.stories.tsx"), story(&upper_kind, &capitalized_name))?;

    Ok(())
}

fn input() -> io::Result<(String, String, String, String)> {
    let kinds = vec!["Atoms", "Molecules", "Organisms", "Templates", "Pages"];
    let upper_kind = kinds[Select::new().items(&kinds).interact()?].to_string();
    let lower_kind = upper_kind.to_ascii_lowercase();

    let hyphenated_name = Input::<String>::new().with_prompt("dir name").interact()?;
    let capitalized_name = hyphenated_name.split('-').map(capitalize).join("");

    Ok((upper_kind, lower_kind, hyphenated_name, capitalized_name))
}

fn root_dir() -> io::Result<String> {
    let stdout = Command::new("git").args(["rev-parse", "--show-toplevel"]).output()?.stdout;
    let root = String::from_utf8(stdout).unwrap();
    Ok(root.trim().to_string())
}

fn tsx(name: &str) -> String {
    format!(
        r#"
import {{ type FC }} from 'react'
import styles from './{name}.module.scss'

interface Props {{
}}

export const {name}: FC<Props> = (props) => {{
  return <>
  </>
}}
    "#
    )
    .trim_start()
    .to_string()
}

fn scss() -> String {
    r#"
.component {
}
    "#
    .to_string()
    .trim_start()
    .to_string()
}

fn story(kind: &str, name: &str) -> String {
    format!(
        r#"
import type {{ Meta, StoryObj }} from '@storybook/react'

import {{ {name} }} from './{name}'

const meta = {{
  title: '{kind}/{name}',
  component: {name},
  tags: ['autodocs'],
  argTypes: {{}},
}} satisfies Meta<typeof {name}>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {{
  args: {{
  }},
}}
    "#
    )
    .trim_start()
    .to_string()
}

fn capitalize(s: &str) -> String {
    let mut cs = s.chars();
    match cs.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + cs.as_str(),
    }
}

fn write<P: AsRef<Path>>(path: P, s: String) -> io::Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{s}")?;
    file.flush()?;
    Ok(())
}
