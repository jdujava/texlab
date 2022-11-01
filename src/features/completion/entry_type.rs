use rowan::{TextRange, TextSize};

use crate::{syntax::bibtex, util::cursor::CursorContext, LANGUAGE_DATA};

use super::types::{InternalCompletionItem, InternalCompletionItemData};

pub fn complete_entry_types<'db>(
    context: &'db CursorContext,
    items: &mut Vec<InternalCompletionItem<'db>>,
) -> Option<()> {
    let range = context
        .cursor
        .as_bib()
        .filter(|token| token.kind() == bibtex::TYPE)
        .map(bibtex::SyntaxToken::text_range)
        .filter(|range| range.start() != context.offset)
        .map(|range| TextRange::new(range.start() + TextSize::from(1), range.end()))?;

    for ty in &LANGUAGE_DATA.entry_types {
        let data = InternalCompletionItemData::EntryType { ty };
        let item = InternalCompletionItem::new(range, data);
        items.push(item);
    }

    Some(())
}
