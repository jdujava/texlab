use crate::feature::FeatureRequest;
use crate::range;
use crate::syntax::bibtex::BibtexDeclaration;
use crate::syntax::latex::{LatexCitationAnalyzer, LatexVisitor};
use crate::syntax::text::SyntaxNode;
use crate::workspace::SyntaxTree;
use lsp_types::{Location, ReferenceParams};

pub struct BibtexEntryReferenceProvider;

impl BibtexEntryReferenceProvider {
    pub async fn execute(request: &FeatureRequest<ReferenceParams>) -> Vec<Location> {
        let mut references = Vec::new();
        if let Some(key) = Self::find_definition(request) {
            for document in &request.related_documents {
                if let SyntaxTree::Latex(tree) = &document.tree {
                    let mut analyzer = LatexCitationAnalyzer::new();
                    analyzer.visit_root(&tree.root);
                    analyzer
                        .citations
                        .iter()
                        .filter(|citation| citation.key.text() == key)
                        .map(|citation| Location::new(document.uri.clone(), citation.command.range))
                        .for_each(|location| references.push(location))
                }
            }
        }
        references
    }

    fn find_definition(request: &FeatureRequest<ReferenceParams>) -> Option<&str> {
        if let SyntaxTree::Bibtex(tree) = &request.document.tree {
            for declaration in &tree.root.children {
                if let BibtexDeclaration::Entry(entry) = declaration {
                    if let Some(key) = &entry.key {
                        if range::contains(key.range(), request.params.position) {
                            return Some(key.text());
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::completion::latex::data::types::LatexComponentDatabase;
    use crate::feature::FeatureSpec;
    use crate::range;
    use crate::test_feature;
    use lsp_types::Position;

    #[test]
    fn test() {
        let references = test_feature!(
            BibtexEntryReferenceProvider,
            FeatureSpec {
                files: vec![
                    FeatureSpec::file("foo.bib", "@article{foo, bar = {baz}}"),
                    FeatureSpec::file("bar.tex", "\\addbibresource{foo.bib}\n\\cite{foo}"),
                    FeatureSpec::file("baz.tex", "\\cite{foo}")
                ],
                main_file: "foo.bib",
                position: Position::new(0, 9),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(
            references,
            vec![Location::new(
                FeatureSpec::uri("bar.tex"),
                range::create(1, 0, 1, 10)
            )]
        );
    }

    #[test]
    fn test_latex() {
        let references = test_feature!(
            BibtexEntryReferenceProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.tex", ""),],
                main_file: "foo.tex",
                position: Position::new(0, 0),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(references, Vec::new());
    }
}
