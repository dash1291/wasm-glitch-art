import { GlitchImage, init_panic_hook } from "wasm-glitch-art";
import { memory } from "wasm-glitch-art/wasm_glitch_art_bg";

let c = document.getElementById('canvas');
let ctx = c.getContext('2d');
init_panic_hook();
console.log(c)
console.log(ctx)
let img;

var onImageReady = function() {
    let Filters = {};
  var glitchImage;
    Filters.getPixels = function(img) {
      var c = this.getCanvas(img.width, img.height)
      
      var ctx = c.getContext('2d');
      ctx.drawImage(img, 0, 0);
      glitchImage = GlitchImage.new(c, ctx);
      const pixelData = new Uint8Array(memory.buffer, glitchImage.raw_pixels(), img.width * img.height * 4);
      pixelData.set(ctx.getImageData(0, 0, c.width,c.height).data, 0)
      return pixelData
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
      var filtered = filter.apply(null, img);
    };
  
    Filters.grayscale = function(pixels, args) {
        let c = document.getElementById('canvas');
        let ctx = c.getContext('2d');
        ctx.getImageData(0,0,c.width,c.height);
        glitchImage.glitch_image();
        const canvasData = new Uint8Array(memory.buffer, glitchImage.raw_pixels(), img.width * img.height * 4);
        const imageData = ctx.createImageData(img.width, img.height);
        imageData.data.set(canvasData);
        ctx.putImageData(imageData, 0, 0)
    };
    console.time('glitching')

    Filters.filterImage(Filters.grayscale, img)
    console.timeEnd('glitching')

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