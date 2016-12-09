var buttons = '<button class="exec btn">run</button><button class="reset btn">reset</button>' + "\n";
var result = '<span class="return"></span>';

function addButtons() {
  $('pre code.rust').each(function(i, block){
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
      console.log(output.compiler);
      target.html(output.output);
    });
  });
  $('.reset').click(function () {
    $(this).siblings('code').text($(this).context._code);
    hljs.highlightBlock($(this).siblings('code')[0]);
  })
  $('.versionable').blur(function () {
    console.log('versioning comming soon')
  });
}
formatOutput = function (output) {
  var parts = output.split(/\n/);
  var compiler = parts.shift();
  return {compiler: compiler, output: parts.join('<br>')}
}
