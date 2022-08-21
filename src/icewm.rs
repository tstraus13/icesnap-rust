use std::process::Command;

// Functions
pub fn restore()
{

}

pub fn move_window(x: i32, y: i32)
{
    let output = Command::new("icesh")
        .arg("-f")
        .arg("move")
        .arg(x.to_string())
        .arg(y.to_string())
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    if !output.status.success()
    {
        panic!("There was an issue calling icesh!");
    }
}

// Structs
pub struct FocusedWindow
{
    pub Screen:         Screen,
    pub Properties:     WindowProperties,
    pub SnappedQuarter: bool,
    pub SnappedTop:     bool,
    pub CanSnapTop:     bool,
    pub SnappedBottom:  bool,
    pub CanSnapBottom:  bool,
    pub SnappedLeft:    bool,
    pub CanSnapLeft:    bool,
    pub SnappedRight:   bool,
    pub CanSnapRight:   bool,
}

pub struct Screen
{
    pub Identity:   i8,
    pub Taskbar:    Taskbar,
    pub Geometry:   Geometry,
}

pub struct WindowProperties
{
    pub State:      State,
    pub Border:     Border,
    pub Geometry:   Geometry,
}

pub struct Taskbar
{
    pub Geometry: Geometry,
}

impl Taskbar
{
    pub fn onBottom(&self) -> bool
    {
        self.Geometry.OffsetY == 0
    }

    pub fn onTop(&self) -> bool
    {
        !self.onBottom()
    }
}

pub struct Border
{
    pub Left:   i32,
    pub Right:  i32,
    pub Top:    i32,
    pub Bottom: i32,
}

pub struct State
{
    pub MaximizedHorizontal:    bool,
    pub MaximizedVertical:      bool,
}

impl State
{
    pub fn isMaximized(&self) -> bool
    {
        self.MaximizedHorizontal && self.MaximizedVertical
    }
}

pub struct Geometry
{
    pub Width:      i32,
    pub Height:     i32,
    pub OffsetX:    i32,
    pub OffsetY:    i32,
}
