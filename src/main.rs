use std::fs;
use std::path::{Path, PathBuf};

use std::env;

use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Select, theme::ColorfulTheme};

/// AIエージェント用ルールファイル配置ツール
#[derive(Parser)]
#[command(name = "airs")]
#[command(about = "AIエージェント用ルールファイルを配置するコマンド", long_about = None)]
struct Cli {
    #[arg(long, value_name = "STORE_DIR")]
    store: Option<PathBuf>,
}

/// プロジェクトのルートディレクトリを特定
fn find_project_root(start_dir: &Path) -> Result<PathBuf> {
    let mut current_dir = start_dir.to_path_buf();
    let root_indicators = [".git", ".github"];

    loop {
        for indicator in &root_indicators {
            if current_dir.join(indicator).exists() {
                return Ok(current_dir);
            }
        }

        if !current_dir.pop() {
            break;
        }
    }

    Ok(start_dir.to_path_buf())
}

/// GitHub Copilot用のルールファイルを配置
fn setup_copilot_rules(target_dir: &Path, store_dir: Option<&Path>) -> Result<()> {
    let root_dir = find_project_root(target_dir)
        .context("プロジェクトルートディレクトリの特定に失敗しました")?;

    let target_file = root_dir.join(".github/copilot-instructions.md");

    if let Some(parent) = target_file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context("ディレクトリの作成に失敗しました")?;
        }
    }

    if let Some(store_path) = store_dir {
        if !store_path.exists() {
            return Err(anyhow::anyhow!(
                "指定されたストアディレクトリが存在しません: {}",
                store_path.display()
            ));
        }

        // ストアディレクトリ内のファイル一覧を取得
        let entries = fs::read_dir(store_path).context(format!(
            "ディレクトリの読み取りに失敗しました: {}",
            store_path.display()
        ))?;

        let mut files: Vec<PathBuf> = entries
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.is_file() && (path.extension().map_or(false, |ext| ext == "md")))
            .collect();

        if files.is_empty() {
            return Err(anyhow::anyhow!(
                "指定されたディレクトリに.mdファイルがありません: {}",
                store_path.display()
            ));
        }

        files.sort();

        let items: Vec<String> = files
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("使用するルールファイルを選択してください")
            .default(0)
            .items(&items)
            .interact()
            .context("ファイル選択に失敗しました")?;

        let selected_file = &files[selection];
        let content = fs::read_to_string(selected_file).context(format!(
            "ファイルの読み取りに失敗しました: {}",
            selected_file.display()
        ))?;

        fs::write(&target_file, content).context("ルールファイルの書き込みに失敗しました")?;

        println!("ルールファイルを配置しました: {}", target_file.display());
    } else {
        todo!("デフォルトの動作を決める");
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let current_dir = env::current_dir().context("現在のディレクトリの取得に失敗しました")?;
    setup_copilot_rules(&current_dir, cli.store.as_deref())?;

    Ok(())
}
