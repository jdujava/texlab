use crate::util::cursor::CursorContext;

use super::types::{InternalCompletionItem, InternalCompletionItemData};

pub fn complete_user_commands<'db>(
    context: &'db CursorContext,
    items: &mut Vec<InternalCompletionItem<'db>>,
) -> Option<()> {
    let range = context.cursor.command_range(context.offset)?;
    let token = context.cursor.as_tex()?;

    let db = context.db;
    for document in context
        .workspace
        .related(db, context.distro, context.document)
    {
        if let Some(data) = document.parse(db).as_tex() {
            let text = document.contents(db).text(db);
            for name in data
                .analyze(db)
                .command_name_ranges(db)
                .iter()
                .copied()
                .filter(|range| *range != token.text_range())
                .map(|range| &text[std::ops::Range::<usize>::from(range)])
            {
                items.push(InternalCompletionItem::new(
                    range,
                    InternalCompletionItemData::UserCommand { name },
                ));
            }
        }
    }

    Some(())
}
