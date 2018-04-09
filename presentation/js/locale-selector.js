const LocaleSelector = (function(){
  const klass = function(){
    this.initialize.apply(this, arguments);
  };
  klass.prototype = {
    initialize: function(conf){
      this.localeFiles = conf.locales || {},
      this.defaultLocale = conf.default || "en-US",
      this.el = conf.el;
      this.templates = {
        slide: conf.templates.slide.content,
        localeSelector: conf.templates.selector.content 
      };
      this.reveal = conf.reveal;
      this.start();
    },
    start: function(){
      const location = new URL(window.location);
      const locale = 
        location.searchParams.get("locale") || this.defaultLocale;
      this.setLocale(locale, location);
    },
    setLocale: function(locale, location){
      location = location || new URL(window.location);
      locale = locale || this.defaultLocale;
      const md = this.localeFiles[locale];
      if(md){
        this.clearSlide();
        this.setupSlide(md);
      }
    },
    clearSlide: function(){
      while(this.el.firstChild){
        this.el.removeChild(this.el.firstChild);
      }
    },
    setupSlide: function(md){
      const fragment = document.importNode(this.templates.slide, true);
      this.el.appendChild(fragment);
      const slide = this.el.querySelector(".markdown-slide");
      slide.dataset.markdown = md;
      this.render();
    },
    render: function(){
      Reveal.initialize(this.reveal);
      const observer = new MutationObserver((records, observer) =>{
        if(records.length > 0 && records[0].addedNodes){
          this.addLocaleSelector(records[0].addedNodes[0]);
          this.setDocumentTitle(records[0].addedNodes[0]);
          observer.disconnect();
        }
      });
      observer.observe(this.el, {childList: true});
    },
    addLocaleSelector: function(slide){
      const fragment = document.importNode(this.templates.localeSelector, true);
      slide.appendChild(fragment);
      const select = slide.querySelector("select");
      select.addEventListener("change", e => {
        const locale = select.value;
        const location = new URL(window.location);
        if(locale.length > 0){
          location.searchParams.set("locale", locale);
          window.location = location;
        }
      });
    },
    setDocumentTitle: function(slide){
      var title = slide.querySelector("h1");
      if (title && title.innerText.length) {
        document.title = title.innerText + ' - ' + document.title;
      }
    }
  };

  klass.initialize = function(conf){
    return new klass(conf);
  };

  return klass;
})();
