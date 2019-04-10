package texlab.rename

import kotlinx.coroutines.runBlocking
import org.eclipse.lsp4j.Position
import org.eclipse.lsp4j.Range
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Test
import texlab.OldWorkspaceBuilder

class LatexEnvironmentRenamerTests {
    @Test
    fun `it should rename unmatched environments`() = runBlocking {
        val edit = OldWorkspaceBuilder()
                .document("foo.tex", "\\begin{foo}\n\\end{bar}")
                .rename("foo.tex", 0, 8, "baz")
                .let { LatexEnvironmentRenamer.get(it)!! }

        assertEquals(1, edit.changes.keys.size)
        val changes = edit.changes.getValue(edit.changes.keys.first())
        assertEquals(2, changes.size)
        assertEquals(Range(Position(0, 7), Position(0, 10)), changes[0].range)
        assertEquals("baz", changes[0].newText)
        assertEquals(Range(Position(1, 5), Position(1, 8)), changes[1].range)
        assertEquals("baz", changes[1].newText)
    }

    @Test
    fun `it should not rename unrelated environments`() = runBlocking<Unit> {
        OldWorkspaceBuilder()
                .document("foo.tex", "\\begin{foo}\n\\end{bar}")
                .rename("foo.tex", 0, 5, "baz")
                .let { LatexEnvironmentRenamer.get(it) }
                .also { assertNull(it) }
    }

    @Test
    fun `it should not process BibTeX documents`() = runBlocking<Unit> {
        OldWorkspaceBuilder()
                .document("foo.bib", "\\begin{foo}\n\\end{bar}")
                .rename("foo.bib", 0, 8, "baz")
                .let { LatexEnvironmentRenamer.get(it) }
                .also { assertNull(it) }
    }
}
