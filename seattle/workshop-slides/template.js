module.exports = {
  document: (node) => {
    const fullTitle = node.getAttribute('lecture')
    const title = fullTitle.substring(0, fullTitle.indexOf(':'))
    const subtitle = fullTitle.substring(fullTitle.indexOf(':') + 1)

    return `<!DOCTYPE html>
  <html lang="en">
  <head>
  <meta charset="UTF-8">
  <link href="../asciidoctor.css" rel="stylesheet">
  <link href="../slides.css" rel="stylesheet">
  <link rel="stylesheet"
        href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
  <script>${insertScreenStyle}; insertScreenStyle();</script>
  </head>
  <body>
  <section class="slide title-slide">
  <h1>${node.getDocumentTitle()}</h1>
  <img alt="Rust Logo" src="../rust-logo-blk.svg">
  </section>
  <section class="slide title-slide">
      <h1>${title}<br>${subtitle}</h1>
  </section>
  ${node.getContent()}
  <script>
  ${init.toString()}
  init()
  </script>
  </body>`
  },

  section: (node) => node.getRoles().includes("title-slide") ?
    `
    <section id="slide-${node.index}" class="slide title-slide">
      <h2>${node.getTitle()}</h2>
    </section>`
    : `
  <section id="slide-${node.index}" class="slide ${node.getRoles().join(" ")} ${node.getTitle() == '!' ? 'no-title' : ''}">
    <h3>${node.getTitle()}</h3>
    <div class='slide-content'>
      ${node.getContent()}
    </div>
    <footer>
    <p class="small">${node.index + 1}/${node.parent.blocks.length}</p>
    </footer>
  </section>`,

  paragraph: (node) => `<p class="${node.getRoles().join(" ")}">${node.getContent()}</p>`,

  image: (node) => {
    const width = node.getAttribute('width')
    const height = node.getAttribute('height')
    return `<img class="image ${node.getRoles()}"
                 src="${node.getImageUri(node.getAttribute('target'))}"
                 ${width ? `width=${width}` : ''}
                 ${height ? `height=${height}` : ''}
                 />`
  }
  // listing: (node) => node.getContent(),
}

function init() {
  function firstToken(TOKENS, text) {
    let tag = null;
    let len = undefined;
    let index = text.length;
    for (let [t, re] of Object.entries(TOKENS)) {
      const m = text.match(re)
      if (!m) continue
      if (m.index < index) {
        index = m.index;
        tag = t;
        len = m[0].length;
      }
    }
    if (index == 0) {
      return [tag, Math.max(len, 1)]
    } else {
      return ['text', Math.max(index, 1)]
    }
  }

  function tokenize(TOKENS, text) {
    const res = [];
    while (true) {
      if (text.length == 0) break;
      let [tag, len] = firstToken(TOKENS, text);
      let tok = text.substring(0, len);
      res.push([tag, tok])
      text = text.substring(len);
    }
    return res
  }

  const RUST_TOKENS = {
    number: /\b([0-9][0-9_]*(\.[0-9]+)?|0[xob][_0-9a-zA-Z]+)([iuf](8|16|32|64|128|size))?\b/,
    string: /"[^"\n]*"|b?'.'/,
    lifetime: /'\w+(?!')/,
    kw: /\b(union|static|move|super|type|trait|where|self|impl|true|false|extern|crate|fn|match|in|if|else|while|for|loop|pub|let|return|break|continue|mut|const|ref|struct|enum|use|mod|as|unsafe|dyn)\b/,
    macro: /[a-zA-Z_]+!(?=[(\\[{])/,
    comment: /\/\/.*/,
    prompt: /\$/,
    text: /<\w+[^<]*>|<\/\w+>/,
  };

  const LANG_TOKENS = {
    number: /\b[0-9][0-9_]*(\.[0-9]+)?(d)?\b/,
    string: /(?<!=)"[^"\n]*"(?!>)/,
    kw: /\b(template|typedef|typename|void|in|if|else|while|for|let|return|break|continue|const|struct|enum|use|as|def|private|static)\b/,
    comment: /\/\/.*/,
    macro: /#\w+ .*/,
    text: /<\w+[^<]*>|<\/\w+>/,
  }

  const SH_TOKENS = {
    comment: /#.*/,
    prompt: /\$/,
    text: /<\w+[^<]*>|<\/\w+>/,
  }

  const langs = {
    'rust': RUST_TOKENS,
    'java': LANG_TOKENS,
    'scala': LANG_TOKENS,
    'cpp': LANG_TOKENS,
    'sh': SH_TOKENS,
  }

  for (const [lang, TOKENS] of Object.entries(langs)) {
    for (const elt of document.getElementsByClassName(`language-${lang}`)) {
      let newText = "";
      for (let [tag, text] of tokenize(TOKENS, elt.innerHTML)) {
        if (tag == 'text') {
          newText += text;
          continue;
        }
        newText += `<span class="hl-${tag}">${text}</span>`
      }
      elt.innerHTML = newText
    }
  }

  function currentSlide() {
    const loc = location.hash.replace("#slide-", "");
    if (!loc) return 0;
    return parseInt(loc);
  }

  const nSlides = document.getElementsByClassName('slide').length - 2;

  function go(delta) {
    let slide = currentSlide() + delta;
    slide = Math.max(slide, 0);
    slide = Math.min(slide, nSlides - 1);
    location.hash = "#slide-" + slide;
  }

  document.onkeydown = function (e) {
    switch (e.keyCode) {
      case 37:
        go(-1);
        break;
      case 33:
        go(-10);
        e.preventDefault()
        break;
      case 39:
        go(+1);
        break;
      case 32: // space
        e.preventDefault()
        go(+1);
        break;
      case 34:
        go(+10);
        e.preventDefault()
        break;
    }
  };
}

function insertScreenStyle() {
  if (!navigator.userAgent.includes("HeadlessChrome")) {
    const style = document.createElement('style')
    style.innerHTML = `
      body {
        padding-bottom: 2400px;
      }
      .slide {
        margin-top: 1em;
        border: 1px solid #ba3925;
        padding: 9px;
      }
    `
    const ref = document.querySelector('script')
    ref.parentNode.insertBefore(style, ref)
  }
}

