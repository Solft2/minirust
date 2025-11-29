use std::{collections::HashMap, path::{Path, PathBuf}};

use glob::PatternError;

use crate::{Repository, objects::RGitObjectTypes, staging::StagingArea};

/// Representa as regras de ignore de arquivos em um repositório
/// Um repositório pode ter múltiplos arquivos de ignore.
/// 
/// A chave de `scoped_rules` é o caminho relativo do arquivo de ignore a partir da pasta raíz do projeto
/// (ex: "", "src").
/// O valor é a lista de regras de ignore contidas naquele arquivo.
/// 
pub struct RGitIgnore {
    scoped_rules: HashMap<String, Vec<String>>
}

impl RGitIgnore {
    pub fn new(repo: &Repository) -> Self {
        let mut scoped_rules: HashMap<String, Vec<String>> = HashMap::new();
        let staging_area = StagingArea::new(repo);

        for entry in staging_area.entries {
            let relative_path = entry.path;
            if relative_path.file_name().unwrap_or_default() == Repository::GITIGNORE {
                let ignore_file_object = repo.get_object(&entry.object_hash).unwrap();

                let ignore_file = match ignore_file_object {
                    RGitObjectTypes::Blob(blob) => {
                        String::from_utf8(blob.content).unwrap_or_default()
                    }
                    _ => {
                        String::new()
                    }
                };

                let rules = ignore_file.lines()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty() && !line.starts_with('#'))
                    .collect::<Vec<String>>();

                let key = relative_path.parent()
                    .map(|p| p.to_str().unwrap_or("").to_string());

                if let Some(k) = key {
                    scoped_rules.insert(k, rules);
                }
            }
        }

        RGitIgnore {
            scoped_rules
        }
    }

    pub fn check_ignore(&self, relative_path: &PathBuf) -> bool {
        let relative_components = relative_path.components();

        // Ignoramos tudo da pasta .minigit por padrão
        if relative_components.clone().any(|comp| comp.as_os_str() == Repository::MINIGITDIR) {
            return true;
        }
        
        let mut ignore_dir = relative_path.to_path_buf();
        ignore_dir.pop();

        loop {
            let ignore_dir_str = ignore_dir.to_str();
            let last = ignore_dir.as_os_str().is_empty();

            match ignore_dir_str {
                None => break,
                Some(s) => { 
                    let maybe_ignore = self.check_ignore_rules(s.to_string(), relative_path);
                    
                    match maybe_ignore {
                        Ok(ignore_result_option) => {
                            if let Some(ignore_result) = ignore_result_option {
                                return ignore_result;
                            }
                        }
                        _ => {}
                    }
                    ignore_dir.pop();
                }
            }

            if last {
                break;
            }
        }

        false
    }

    fn check_ignore_rules(&self, key: String, relative_path: &Path) -> Result<Option<bool>, PatternError> {
        println!("Rules: {:?}", self.scoped_rules);
        let mut last_result: Option<bool> = None;
        let scoped_rules = self.scoped_rules.get(&key);

        match scoped_rules {
            None => return Ok(None),
            Some(rules) => {
                for rule in rules{
                    let result = Self::mathes_rule(&key, rule, relative_path)?;
                    last_result = Some(result);
                }
            }
        }
        
        Ok(last_result)
    }

    fn mathes_rule(key: &String, rule: &String, relative_path: &Path) -> Result<bool, PatternError> {
        let prefix = if key.is_empty() {
            String::new()
        } else {
            format!("{}/", key)
        };

        if rule.starts_with('!') {
            let glob_str = format!("{}{}", &prefix, &rule[1..]);
            let glob_str_dir = format!("{}{}/**/*", &prefix, &rule[1..]);
            let matches = glob::Pattern::new(&glob_str)?.matches_path(relative_path);
            let matches_dir = glob::Pattern::new(&glob_str_dir)?.matches_path(relative_path);

            if matches || matches_dir {
                return Ok(false);
            }
        } else {
            let glob_str = format!("{}{}", &prefix, rule);
            let glob_str_dir = format!("{}{}/**/*", &prefix, rule);

            let matches = glob::Pattern::new(&glob_str)?.matches_path(relative_path);
            let matches_dir = glob::Pattern::new(&glob_str_dir)?.matches_path(relative_path);

            if matches || matches_dir {
                return Ok(true);
            }
        }

        Ok(false)
    }
}