
function drawSvgs() {
  $('pre.diagram').each(function(i, block){
      var svg = Module.process_string("demo", block.textContent, "rust diagram")
      var parent = block.parentNode;
      var svgElement = htmlToElement(svg);
      parent.removeChild(block);
      parent.appendChild(svgElement);
  })
}

function htmlToElement(html) {
    var template = document.createElement('template');
    template.innerHTML = html;
    return template.content.firstChild;
}
