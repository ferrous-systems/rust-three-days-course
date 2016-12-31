joint.shapes.memory = {};

joint.shapes.memory.Field = joint.shapes.basic.Generic.extend({
    markup: [
        '<g class="rotatable">',
        '<g class="scalable">',
        '<rect class="field-name-rect"/><rect class="field-type-rect"/><rect class="field-size-rect"/>',
        '</g>',
        '<text class="field-name-text"/><text class="field-type-text"/><text class="field-size-text"/>',
        '</g>'
    ].join(''),

    defaults: _.defaultsDeep({

        type: 'memory.Field',

        attrs: {
            rect: { 'width': 200 },

            '.field-name-rect': { 'stroke': 'black', 'stroke-width': 2, 'fill': '#3498db' },
            '.field-type-rect': { 'stroke': 'black', 'stroke-width': 2, 'fill': '#3498db' },
            '.field-size-rect': { 'stroke': 'black', 'stroke-width': 2, 'fill': '#3498db' },

            '.field-name-text': {
                'ref': '.field-name-rect', 'ref-y': .5, 'ref-x': .5, 'text-anchor': 'middle', 'y-alignment': 'middle', 'font-weight': 'bold',
                'fill': 'black', 'font-size': 12, 'font-family': 'Times New Roman'
            },
            '.field-type-text': {
                'ref': '.field-type-rect', 'ref-y': .5, 'ref-x': .5, 'text-anchor': 'middle', 'y-alignment': 'middle',
                'fill': 'black', 'font-size': 12, 'font-family': 'Times New Roman'
            },
            '.field-size-text': {
                'ref': '.field-size-rect', 'ref-y': .5, 'ref-x': .5, 'text-anchor': 'middle', 'y-alignment': 'middle',
                'fill': 'black', 'font-size': 12, 'font-family': 'Times New Roman'
            }
        },

        name: [],
        type: [],
        fieldSize: "?"
    }, joint.shapes.basic.Generic.prototype.defaults),


    initialize: function() {
        this.on('change:type change:name', function() {
            this.updateRectangles();
            this.trigger('memory-update');
        }, this);

        this.updateRectangles();

        joint.shapes.basic.Generic.prototype.initialize.apply(this, arguments);
    },

    getFieldName: function() {
        return this.get('name');
    },

    getTypeName: function() {
        return this.get('type');
    },

    getFieldSize: function() {
        return this.get('fieldSize');
    },

    updateRectangles: function() {

        var attrs = this.get('attrs');

        var rects = [
            { type: 'name', text: this.getFieldName(), offset: 0 },
            { type: 'type', text: this.getTypeName(), offset: 200 },
            { type: 'size', text: this.getFieldSize(), offset: 400 }
        ];

        var offsetY = 0;

        _.each(rects, function(rect) {
            var lines = _.isArray(rect.text) ? rect.text : [rect.text];
            var rectHeight = lines.length * 20 + 20;

            attrs['.field-' + rect.type + '-text'].text = lines.join('\n');
            attrs['.field-' + rect.type + '-rect'].height = rectHeight;
            attrs['.field-' + rect.type + '-rect'].transform = 'translate(' + rect.offset + ',0)';

            offsetY += rectHeight;
        });
    }
});

joint.shapes.memory.FieldView = joint.dia.ElementView.extend({

    initialize: function() {

        joint.dia.ElementView.prototype.initialize.apply(this, arguments);

        this.listenTo(this.model, 'memory-update', function() {
            this.update();
            this.resize();
        });
    }
});

joint.shapes.memory.Struct = joint.shapes.basic.Generic.extend({
    markup: [
        '<g class="rotatable">',
        '<g class="scalable">',
        '<rect class="type-name-rect"/>',
        '</g>',
        '<text class="type-name-text"/>',
        '</g>'
    ].join(''),

    defaults: _.defaultsDeep({

        type: 'memory.Field',

        attrs: {
            rect: { 'width': 200 },

            '.type-name-rect': { 'stroke': 'black', 'stroke-width': 2, 'fill': '#3498db' },

            '.type-name-text': {
                'ref': '.type-name-rect', 'ref-y': .3, 'ref-x': .1, 'font-weight': 'bold',
                'fill': 'black', 'font-size': 12, 'font-family': 'Times New Roman'
            }
        },

        name: []
    }, joint.shapes.basic.Generic.prototype.defaults),


    initialize: function() {
        this.on('change:name change:fields', function() {
            this.updateRectangles();
            this.trigger('memory-update');
        }, this);

        this.updateRectangles();

        joint.shapes.basic.Generic.prototype.initialize.apply(this, arguments);
    },

    getClassName: function() {
        return this.get('name');
    },

    updateRectangles: function() {

        var attrs = this.get('attrs');

        var rects = [
            { type: 'name', text: this.getClassName() }
        ];

        var offsetY = 0;

        _.each(rects, function(rect) {
            var lines = _.isArray(rect.text) ? rect.text : [rect.text];
            var rectHeight = lines.length * 20 + 20;

            attrs['.type-' + rect.type + '-text'].text = lines.join('\n');
            attrs['.type-' + rect.type + '-rect'].height = rectHeight;
            attrs['.type-' + rect.type + '-rect'].transform = 'translate(0,' + offsetY + ')';

            offsetY += rectHeight;
        });
    }
});

joint.shapes.memory.StructView = joint.dia.ElementView.extend({

    initialize: function() {

        joint.dia.ElementView.prototype.initialize.apply(this, arguments);

        this.listenTo(this.model, 'memory-update', function() {
            this.update();
            this.resize();
        });
    }
});
