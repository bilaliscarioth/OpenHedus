extern crate x11;
extern crate gl;


use x11::xlib::{Display, Window, Screen, XEvent, XDefaultScreenOfDisplay, XCloseDisplay,
                XWindowAttributes, XOpenDisplay, XDefaultScreen, XVisualInfo, XFree,
                XSetWindowAttributes, XBlackPixel, XWhitePixel, True, False,
                XCreateColormap, AllocNone, XRootWindow, ExposureMask, KeyPressMask,
                KeyReleaseMask, XCreateWindow, InputOutput, CWBackPixel, CWColormap, 
                CWBorderPixel, CWEventMask, XStoreName, XSync, XMapRaised, XClearWindow, 
                XMapWindow, XGetWindowAttributes, Visual};

use x11::glx::{GLXContext, GLXFBConfig, glXQueryVersion, glXGetVisualFromFBConfig,
                GLX_RGBA, GLX_DEPTH_SIZE, GLX_DOUBLEBUFFER, GLX_NONE, glXChooseFBConfig,
                glXGetFBConfigAttrib, GLX_SAMPLE_BUFFERS, GLX_SAMPLES, glXGetProcAddressARB,
                glXQueryExtensionsString,  glXCreateNewContext, glXMakeCurrent,
                GLX_RGBA_TYPE, glXSwapBuffers};

use x11::glx::arb::{GLX_CONTEXT_MAJOR_VERSION_ARB, GLX_CONTEXT_MINOR_VERSION_ARB,
                    GLX_CONTEXT_FLAGS_ARB, GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB};

use gl::types::{GLint};

use std::ffi::c_void;
use std::ptr;
use std::mem::transmute;
use std::ffi::CStr;


type GlXcreateContextAttribsArbproc = fn(*mut Display, GLXFBConfig, GLXContext, bool, *mut u32) -> *mut GLXContext;

pub struct LseWindow{
    display :   *mut Display,
    window  :   Window,
    screen  :   *mut Screen,
    screenId:   i32,
    xev     :   *mut XEvent,
    attribs :   *mut XWindowAttributes,
    GlXcreateContextAttribsArb : GlXcreateContextAttribsArbproc,
    context :   *mut GLXContext
}

impl LseWindow{
    pub fn new(_title: &str, _width: u16, _height: u16) -> Result<LseWindow, throw::Error<&'static str>> {
        let mut tmpSelf : LseWindow = LseWindow{ 
            display:    ptr::null_mut() as *mut Display,
            window:     0,
            screen:     ptr::null_mut() as *mut Screen,
            screenId :  0,
            xev:        ptr::null_mut() as *mut XEvent,
            attribs :   ptr::null_mut() as *mut XWindowAttributes,
            GlXcreateContextAttribsArb:  unsafe {
                transmute::<Option<unsafe extern "C" fn()>, GlXcreateContextAttribsArbproc >
                (glXGetProcAddressARB("glXCreateContextAttribsARB".as_ptr())) },
            context :   ptr::null_mut() as *mut GLXContext
        };
        
        tmpSelf.display = unsafe {  XOpenDisplay(ptr::null()) };

        if tmpSelf.display.is_null() {
            throw_new!("Impossible to open display"); 
        }

        tmpSelf.screen = unsafe { XDefaultScreenOfDisplay(tmpSelf.display)  };
        tmpSelf.screenId = unsafe {  XDefaultScreen(tmpSelf.display) };

        let mut majorGLX : GLint = 0;
        let mut minorGLX : GLint = 0;

        unsafe {
            glXQueryVersion(tmpSelf.display, &mut majorGLX, &mut minorGLX);
        };

        if majorGLX <= 1 && minorGLX < 2 {
            throw_new!("Old GLX version not supported");
        }
        
        let glxAttribs : [GLint; 5] = [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, GLX_NONE];
        let mut fbcount : i32 = 0;

        let fbc: *mut GLXFBConfig =  unsafe { glXChooseFBConfig(tmpSelf.display, tmpSelf.screenId, glxAttribs.as_ptr(), &mut fbcount) };

        if fbc.is_null(){
            unsafe { XCloseDisplay(tmpSelf.display); };
            throw_new!("Not found a good FBconfig");
        }


        let mut best_fbc : i32 = -1;
        let mut worst_fbc : i32 = -1;
        let mut best_num_samp : i32 = -1;
        let mut worst_num_samp : i32 = 999;
        
        for i in 0..fbcount  {
            let vi : *mut XVisualInfo = unsafe { glXGetVisualFromFBConfig(tmpSelf.display, *fbc.offset(i.try_into().unwrap())) };
            if !vi.is_null() {
                let mut samp_buf : i32 = 0;
                let mut samples : i32 = 0;

                unsafe {
                    glXGetFBConfigAttrib( tmpSelf.display, *fbc.offset(i.try_into().unwrap()) , 
                        GLX_SAMPLE_BUFFERS, &mut samp_buf);
                    glXGetFBConfigAttrib( tmpSelf.display, *fbc.offset(i.try_into().unwrap()) ,  
                        GLX_SAMPLES, &mut samples);
                };

                if best_fbc < 0 ||  (samp_buf > 0 && samples > best_num_samp) {
                    best_fbc = i;
                    best_num_samp = samples;
                } 

                if worst_fbc < 0 || !samp_buf > 0 || samples < worst_num_samp {
                    worst_fbc = i;
                } 
                worst_num_samp = samples;
            }
            unsafe {
                XFree(vi as *mut c_void);
            };
        }

        let bestFbc : GLXFBConfig = unsafe { * fbc.offset(best_fbc.try_into().unwrap()) };

        unsafe { 
            XFree( fbc as *mut c_void );
        }

        let visual : *mut XVisualInfo = unsafe {
            glXGetVisualFromFBConfig(tmpSelf.display, bestFbc)
        };

        if visual.is_null() {
            unsafe { XCloseDisplay(tmpSelf.display); };
            throw_new!("Can't create correct visual window");
        }

        if tmpSelf.screenId != unsafe { (*visual).screen } {
            unsafe { XCloseDisplay(tmpSelf.display); };
            throw_new!("Visual doesn't match with tmpScreenId");
        }

        let mut windowAttribs : XSetWindowAttributes = XSetWindowAttributes {
            border_pixel : unsafe {  XBlackPixel(tmpSelf.display, tmpSelf.screenId) },
            background_pixel :  unsafe { XWhitePixel(tmpSelf.display, tmpSelf.screenId) },
            override_redirect : True,
            colormap : unsafe { XCreateColormap(tmpSelf.display, XRootWindow(tmpSelf.display, tmpSelf.screenId), (*visual).visual, AllocNone) },
            event_mask : ExposureMask | KeyPressMask | KeyReleaseMask,
            background_pixmap: 0,
            bit_gravity: 0,
            win_gravity: 0,
            save_under: 0,
            do_not_propagate_mask: 0,
            cursor: 0,
            backing_pixel:  0,
            backing_planes: 0,
            backing_store: 0,
            border_pixmap: 0
        };

        tmpSelf.window = unsafe { 
            XCreateWindow(tmpSelf.display, XRootWindow(tmpSelf.display, tmpSelf.screenId), 0, 0, _width.into()
            , _height.into() , 0, (*visual).depth, InputOutput.try_into().unwrap(), (*visual).visual, 
            CWBackPixel | CWColormap | CWBorderPixel | CWEventMask, &mut windowAttribs)
        };
        tmpSelf.set_title(_title);


        let contextAttribs : [i32; 7]  =  [ 
            GLX_CONTEXT_MAJOR_VERSION_ARB, 3, 
            GLX_CONTEXT_MINOR_VERSION_ARB, 3,
            GLX_CONTEXT_FLAGS_ARB, GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
            GLX_NONE
        ];

        unsafe {
            let glxExts : &str =  CStr::from_ptr(glXQueryExtensionsString(tmpSelf.display, tmpSelf.screenId)).to_str().unwrap();
            tmpSelf.context = &mut glXCreateNewContext( tmpSelf.display, bestFbc, GLX_RGBA_TYPE, ptr::null_mut() as GLXContext, True );
        };
        
        unsafe {
            XSync( tmpSelf.display, 0);
            glXMakeCurrent(tmpSelf.display, tmpSelf.window, *tmpSelf.context);
            // Show the window
            XClearWindow(tmpSelf.display, tmpSelf.window);
            XMapRaised(tmpSelf.display, tmpSelf.window);

            gl_loader::init_gl();
            gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
            gl::Enable(gl::DEPTH);
        };
        
        Ok(tmpSelf)
    }

    pub fn refresh(&self) {
        unsafe {
            let mut attribs : XWindowAttributes = XWindowAttributes {
                x:  0,
                y:  0,
                width:  0,
                height: 0,
                border_width:   0,
                depth:  0,
                visual: ptr::null_mut() as *mut Visual,
                root:   0,
                class:  0,
                bit_gravity:    0,
                win_gravity:    0,
                backing_store:     0,
                backing_planes:     0,
                backing_pixel:   0,
                save_under: 0,
                colormap:   0,
                map_installed:  0,
                map_state:  0,
                all_event_masks:    0,
                your_event_mask:    0,
                do_not_propagate_mask:   0,
                override_redirect:  0,
                screen: ptr::null_mut() as *mut Screen
            };
            XGetWindowAttributes(self.display, self.window, &mut attribs);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

    }

    pub fn is_fullscreen(&self){
    
    }

    pub fn swap_buffer(&self){
        unsafe {
            glXSwapBuffers(self.display, self.window);                        
        };
    }

    pub fn set_title(&self, _title: &str) {
        unsafe {
            XStoreName(self.display, self.window, _title.as_ptr() as *const i8);
        };

    }

}
