

// function includeHTML() {
//   var z, i, elmnt, file, xhttp;
//   /*loop through a collection of all HTML elements:*/
//   z = document.getElementsByTagName("*");
//   for (i = 0; i < z.length; i++) {
//     elmnt = z[i];
//     /*search for elements with a certain atrribute:*/
//     file = elmnt.getAttribute("include-html");
//     if (file) {
//       /*make an HTTP request using the attribute value as the file name:*/
//       xhttp = new XMLHttpRequest();
//       xhttp.onreadystatechange = function() {
//         if (this.readyState == 4) {
//           if (this.status == 200) {elmnt.innerHTML = this.responseText;}
//           if (this.status == 404) {elmnt.innerHTML = "Page not found.";}
//
//           /*remove the attribute, and call this function once more:*/
//           elmnt.removeAttribute("include-html");
//           includeHTML();
//         }
//       }
//       xhttp.open("GET", file, true);
//       xhttp.send();
//       /*exit the function:*/
//       return;
//     }
//   }
// };

fetchAllCode = function(){
	Array.prototype.slice.call(document.querySelectorAll('code[data-source]')).forEach(function(codeContainer){
		console.log(codeContainer.dataset.source);
		var xhr = new XMLHttpRequest();
		xhr.open("GET", codeContainer.dataset.source, true);
		xhr.overrideMimeType("text/plain; charset=UTF-8");
		xhr.onreadystatechange = function () {
			if (xhr.readyState == 4) {
				if (xhr.status == 200) {
					var code = document.createTextNode(xhr.responseText);
					codeContainer.appendChild(code);
					if(typeof(hljs) !== 'undefined') {
						hljs.highlightBlock(codeContainer);
					}
				}
				else
				{
					console.error(xhr.status)
					console.error("Error while trying to get remote code");
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
