var buttons = '<button class="exec btn">run</button><button class="reset btn">reset</button><button class="open-in-playground btn">open</button>' + "\n";
var result = '<span class="return"></span>';

function addButtons() {
  $('pre code.rust').each(function(i, block){
    $(block).before(buttons);
    $(block).after(result);
  });
  $('pre code.lang-rust').each(function(i, block){
    $(block).before(buttons);
    $(block).after(result);
  });
  $('.reset').each(function (n) {
    $(this).context._code = $(this).siblings('code').text();
  });
  $('.exec').click(function () {
    var target = $(this).siblings('.return');
    target.html('<img src="img/rust.gif" style="border:none; box-shadow:none; margin: 0; background: none;">');
    var code = $(this).siblings('code').text();
    var payload = {optimize:"0", version:"stable", code: code};
    $.ajax({
      url: 'https://play.rust-lang.org/evaluate.json',
      type: "POST",
      dataType: "json",
      data: JSON.stringify(payload),
      contentType: "application/json"
    }).done(function(result) {
      var output = formatOutput(result.result);
      console.log(result.error);
      target.html(output);
    });
  });
  $('.reset').click(function () {
    $(this).siblings('code').text($(this).context._code);
    hljs.highlightBlock($(this).siblings('code')[0]);
  })
  $('.open-in-playground').click(function () {
    var code = $(this).siblings('code').text();
    var baseUrl = 'https://play.rust-lang.org/?version=stable&code=';
    var code = extendCode(code);
    var payload = encodeURIComponent(code);
    var url = baseUrl + payload;

    window.open(url, '_blank');
  });
  $('.versionable').blur(function () {
    console.log('versioning comming soon')
  });
}
formatOutput = function (output) {
  var parts = output.split(/\n/);
  return parts.join('<br>');
}

extendCode = function (code) {
  window.code = code
  console.log(code)
  console.log("matching fn", code.match(/^fn \w+/))
  console.log("matching main", code.match(/^fn main/))
  // No functions, wrap all in main
  if (!code.match(/^fn \w+/m)) {
    code = "fn main() {\n" + code + "\n}"
  } else if (!code.match(/^fn main/m)) { // some functions, no main, add an empty one
    code = code + "\n\nfn main() {}";
  }
  return code;
}
