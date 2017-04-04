fetchAllDiagrams = function(){
	Array.prototype.slice.call(document.querySelectorAll('pre[class="diagram"][data-source]')).forEach(function(diagramContainer){
		console.log(diagramContainer.dataset.source);
		var xhr = new XMLHttpRequest();
		xhr.open("GET", diagramContainer.dataset.source, true);
		xhr.overrideMimeType("text/plain; charset=UTF-8");
		xhr.onreadystatechange = function () {
			if (xhr.readyState == 4) {
				if (xhr.status == 200) {
                    processDiagram(diagramContainer, xhr.responseText);
				}
				else
				{
					console.error(xhr.status)
					console.error("Error while trying to get remote diagram");
				}
			}
		};
		try {
			xhr.send(null);
		} catch (e) {
			console.error("XHR failed for " + url + ", " + e);
		}
	});
}

processDiagram = function(container, content) {
    var svg = Module.process_string("demo", content, "rust diagram")
    var parent = container.parentNode;

    var template = document.createElement('template');
    template.innerHTML = svg;
    var svgElement = template.content.firstChild;

    parent.removeChild(container);
    parent.appendChild(svgElement);
    
}