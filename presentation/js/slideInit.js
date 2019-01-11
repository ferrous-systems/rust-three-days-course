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
        "en-US": "chapters/en-US/" + chapter + ".chapter",
        "de-DE": "chapters/de-DE/" + chapter + ".chapter",
        "es-ES": "chapters/es-ES/" + chapter + ".chapter",
	"fr-Fr": "chapters/fr-FR/" + chapter + ".chapter"
    },
    default: "de-DE",
    reveal: {
        history: true,
        dependencies: [
 	         { src: 'components/reveal.js/plugin/markdown/marked.js' },
 	         { src: 'components/reveal.js/plugin/markdown/markdown.js' },
 	         { src: 'components/reveal.js/plugin/notes/notes.js', async: true },
 	         { src: 'components/reveal.js/plugin/highlight/highlight.js', async: true, callback: function() { fetchAllCode(); hljs.initHighlightingOnLoad(); addButtons(); } },
             { src: 'js/remote-diagrams.js', async: true, callback: function() { fetchAllDiagrams(); } }
        ]
    }
});
