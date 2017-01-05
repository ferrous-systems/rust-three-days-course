var urlParams = new URLSearchParams(window.location.search);
var chapter;
if (urlParams.has("chapter")) {
    chapter = urlParams.get("chapter");
} else {
    chapter = "intro";
}
LocaleSelector.initialize({
    templates: {
        slide: document.querySelector("#markdown-section"),
        selector: document.querySelector("#locale-selector")
    },
    el: document.querySelector(".slides"),
    locales: {
        "en-US": "chapters/en-US/" + chapter + ".md",
        "de-DE": "chapters/de-DE/" + chapter + ".md"
    },
    default: "de-DE",
    reveal: {
        history: true,
        dependencies: [
 	         { src: 'components/reveal.js/plugin/markdown/marked.js' },
 	         { src: 'components/reveal.js/plugin/markdown/markdown.js' },
 	         { src: 'components/reveal.js/plugin/notes/notes.js', async: true },
 	         { src: 'components/reveal.js/plugin/highlight/highlight.js', async: true, callback: function() { fetchAllCode(); hljs.initHighlightingOnLoad(); addButtons(); } },
            { src: 'js/svgDrawings.js', async: true, callback: function() { drawSvgs() } }
        ]
    }
});
