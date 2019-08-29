import { GlitchImage, init_panic_hook } from "wasm-glitch-art";


let c = document.getElementById('canvas');
let ctx = c.getContext('2d');
init_panic_hook();
console.log(c)
console.log(ctx)
let img;

var onImageReady = function() {
    let Filters = {};
    Filters.getPixels = function(img) {
      var c = this.getCanvas(img.width, img.height)
      var ctx = c.getContext('2d');
      console.log(img)
      ctx.drawImage(img, 0, 0);
      return ctx.getImageData(0,0,c.width,c.height);
    };
  
    Filters.getCanvas = function(w,h) {
      console.log('canvasing')
      var c = document.getElementById('canvas');
      c.width = w;
      c.height = h;
      return c;
    };
  
    Filters.filterImage = function(filter, image) {
      var args = [this.getPixels(image)]

      //ctx.putImageData(this.getPixels[image], 0, 0)

      var filtered = filter.apply(null, img);
      //var c = this.getCanvas(img.width, img.height)
      //var ctx = c.getContext('2d');
    };
  
    Filters.grayscale = function(pixels, args) {
        let c = document.getElementById('canvas');
        let ctx = c.getContext('2d');
        const a = GlitchImage.new(c, ctx);
        a.glitch_image();
        a.paint_image(c, ctx);
    };
  
    Filters.filterImage(Filters.grayscale, img)
  }
  
  function handleImage(e){
      var reader = new FileReader();
      reader.onload = function(event){
          img = new Image();
          img.onload = function(){
            onImageReady();
          }
          img.src = event.target.result;
      }
      reader.readAsDataURL(e.target.files[0]);
  }
  
  var imageControl = document.getElementById('imageControl');
  imageControl.onchange = handleImage;