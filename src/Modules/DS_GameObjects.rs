// SET: Script Info
/*<I>:
© IVN - Iterik Viscela Nova, © Matěj Zahradník, © TERA_Ventl, Open Source
Development Standard:	IVN_ATK-ATL_Provisions:TERA:762:XR
------------------------------
Project leader:		-
Supervisor:			Matěj Zahradník [The_Universality] - © Matěj Zahradník
Scripted by: 		Matěj Zahradník [The_Universality] - © Matěj Zahradník
------------------------------
Created at:			15:09 | 03.06.2025 [D.M.Y]
------------------------------
Script type:		Definition Script (DS)
Script purpose:		Defines GameObjects
:<I>*/

// Script Info !REM
//----------------------------
// SET: Compliler messages

//<I>:
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
//:<I>

// Compliler messages !REM
//----------------------------
// SET: Package and Import statements

//<S>:
use std;
//:<S>

// Package and Import statements !REM
//----------------------------
// SET: ENUMs / STRUCs

/*CAT: nDSpaceSide
A struct of side types for 3D & 2D based game objects
Bool within each nDSpaceSide determides whether side is --relative to object rotation-- ``value --true--`` or --absolute to space-- ``value --false--``
<S>:*/
pub enum nDSpaceSide6
{
	Top(bool),
	Bottom(bool),
	Front(bool),
	Back(bool),
	Left(bool),
	Right(bool),
}
//:<S>

/*CAT:Primitive2DShape
A struct of Primitive shapes for ['Primitive2D'] objects
<S>:*/
pub enum Primitive2DShape
{
	Rectangle,
	Circle
}
//:<S>

/*CAT:Primitive3DShape
A struct of Primitive shapes for ['Primitive3D'] objects
<S>:*/
pub enum Primitive3DShape
{
	Cube,
	Ball,
	Cilinder
}
//:<S>

//----------------------------

// @TheUniversality descriptions needed here later on
pub struct Color
{
	pub R: f32,
	pub G: f32,
	pub B: f32,
	pub A: f32
}

pub struct Vector2
{
	pub X: f32,
	pub Y: f32
}


pub struct Vector3
{
	pub X: f32,
	pub Y: f32,
	pub Z: f32
}


pub struct Point2D
{
	pub Position: Vector2,
	pub ZIndex: u8
}


pub struct Point3D
{
	pub Position: Vector3
}


pub struct Object2D
{
	pub Position: Vector2,
	pub Size: Vector2,
	pub Rotation: Vector2,
	pub ZIndex: u8,
}

//Cat:Object3D
/*--Object3D-- is a base for all 3D ``based`` objects like primitive, mesh, etc
It composes of:
<T>:Name,Type,Usage
Position,Vector3,The object position in 3D space
Size,Vector3,The object size in 3D space
Rotation,Vector3,The object rotation in 3D space
:<T>
<S>:*/
pub struct Object3D
{
	pub Position: Vector3,
	pub Size: Vector3,
	pub Rotation: Vector3,
}
//:<S>

//Cat:Primitive2D
/*--Primitive2D-- is a 2D ``based`` object that has a simple rendering shape
It composes of:
<T>:Name,Type,Usage
Position,Vector3,The object position in 3D space
Size,Vector3,The object's size in 3D space
Color,Color `` @ClassKsune - You need to tell me whether it should be set for the whole object as is now or for each side Array[4] ([6] for 3D) of Color ``
:<T>
<S>:*/
pub struct Primitive2D
{
	pub Base: Object2D,
	pub Shape: Primitive2DShape,
	pub Color: Color
}
//:<S>


pub struct Primitive3D
{
	pub Base: Object3D,
	pub Shape: Primitive3DShape,
	pub Color: Color
}

// ENUMs / STRUCs !REM
//----------------------------
// SET: Implementations

//CAT: Color implementation
//<LM>
impl Color
{
	/*<F>:PRT:New_FromRGB
	--Description--: Creates a new value of ['Color'] using RGB --u8-- data type - Color int value 0-255 is converted to float 0.0 - 1.0 ``(I higher value is passed it gets changed to the maximal value)``
	--Returns--: Color
	--Parameters--:
	<T>:	DataType,Name,Usage
			u8,PAR_R_U8,Red color value
			u8,PAR_G_U8,Green color value
			u8,PAR_B_U8,Blue color value
			u8,PAR_Opacity_U8,Opacity value ``0.0 - Color can be seen, 1.0 - Color is invisible (opposite of transparency)``
	:<T>:<F>
	<S>:*/
	pub fn New_FromRGB(PAR_R_U8: u8, PAR_G_U8: u8, PAR_B_U8: u8, PAR_Opacity_U8: u8) -> Color
	{
		Color
		{
			R:
			if(PAR_R_U8 > 255)
			{
				1.0_f32
			}
			else
			{
				(255/PAR_R_U8) as f32
			},
			G:
			if(PAR_G_U8 > 255)
			{
				1.0_f32
			}
			else
			{
				(255/PAR_G_U8) as f32
			},
			B:
			if(PAR_B_U8 > 255)
			{
				1.0_f32
			}
			else
			{
				(255/PAR_B_U8) as f32
			},
			A:
			if(PAR_Opacity_U8 > 255)
			{
				1.0_f32
			}
			else
			{
				(255/PAR_Opacity_U8) as f32
			},
		}
	}
	//:<S>

	//---
	/*<F>:PRT:New
	--Description--: Creates a new value of ['Color'] using --f32-- data type - Color representation are values between 0.0 & 1.0 ``(I higher value is passed it gets changed to the maximal value)``
	--Returns--: Color
	--Parameters--:
	<T>:	DataType,Name,Usage
			f32,PAR_R_F32,Red color value
			f32,PAR_G_F32,Green color value
			f32,PAR_B_F32,Blue color value
			f32,PAR_Opacity_F32,Opacity value ``0.0 - Color can be seen, 1.0 - Color is invisible (opposite of transparency)``
	:<T>:<F>
	<S>:*/
	pub fn New(PAR_R_F32: f32, PAR_G_F32: f32, PAR_B_F32: f32, PAR_Opacity_F32: f32) -> Color
	{
		Color
		{
			R:
			if(PAR_R_F32 > 1.0_f32)
			{
				1.0_f32
			}
			else
			{
				PAR_R_F32
			},
			G:
			if(PAR_G_F32 > 1.0_f32)
			{
				1.0_f32
			}
			else
			{
				PAR_G_F32
			},
			B:
			if(PAR_B_F32 > 1.0_f32)
			{
				1.0_f32
			}
			else
			{
				PAR_B_F32
			},
			A:
			if(PAR_Opacity_F32 > 1.0_f32)
			{
				1.0_f32
			}
			else
			{
				PAR_Opacity_F32
			},
		}
	}
	//:<S>
	//---
}

//CAT: Vector2 implementation
//<LM>
impl Vector2
{
	/*<F>:PRT:New
	--Description--: Creates a new instance of ['Vector2'] using --isize-- data type
	--Returns--: Vector2
	--Parameters--:
	<T>:	DataType,Name,Usage
			isize,PAR_X_Isize,X value
			isize,PAR_Y_Isize,Y value
	:<T>:<F>
	<S>:*/
	pub fn New(PAR_X_Isize: isize, PAR_Y_Isize: isize) -> Vector2
	{
		Vector2 { X: PAR_X_Isize as f32, Y: PAR_Y_Isize as f32 }
	}
	//:<S>

	//---
	/*<F>:PRT:New_FromF32
	--Description--: Creates a new instance of ['Vector2'] using --f32-- data type
	--Returns--: Vector2
	--Parameters--:
	<T>:	DataType,Name,Usage
			f32,PAR_X_F32,X value
			f32,PAR_Y_F32,Y value
	:<T>:<F>
	<S>:*/
	pub fn New_FromF32(PAR_X_F32: f32, PAR_Y_F32: f32) -> Vector2
	{
		Vector2 { X: PAR_X_F32, Y: PAR_Y_F32 }
	}
	//:<S>
	//---
}

//CAT: Vector3 implementation
//<LM>
impl Vector3
{
	/*<F>:PRT:New
	--Description--: Creates a new instance of ['Vector3'] using --isize-- data type
	--Returns--: Vector3
	--Parameters--:
	<T>:	DataType,Name,Usage
			isize,PAR_X_Isize,X value
			isize,PAR_Y_Isize,Y value
			isize,PAR_Z_Isize,Z value
	:<T>:<F>
	<S>:*/
	pub fn New(PAR_X_Isize: isize, PAR_Y_Isize: isize, PAR_Z_Isize: isize) -> Vector3
	{
		Vector3 { X: PAR_X_Isize as f32, Y: PAR_Y_Isize as f32, Z: PAR_Z_Isize as f32}
	}
	//:<S>
	//---
	/*<F>:PRT:New_FromF32
	--Description--: Creates a new instance of ['Vector3'] using --f32-- data type
	--Returns--: Vector3
	--Parameters--:
	<T>:	DataType,Name,Usage
			f32,PAR_X_F32,X value
			f32,PAR_Y_F32,Y value
			f32,PAR_Z_F32,Z value
	:<T>:<F>
	<S>:*/
	pub fn New_FromF32(PAR_X_F32: f32, PAR_Y_F32: f32, PAR_Z_F32: f32) -> Vector3
	{
		Vector3 { X: PAR_X_F32, Y: PAR_Y_F32, Z: PAR_Z_F32 }
	}
	//:<S>
	//---
}


//CAT: Object2D implementation
//<LM>
impl Object2D
{
	/*<F>:PRT:New_Default
	--Description--: Creates a new instance of ['Object2D'] at default rendering ZIndex layer 10, default 2D size (1,1) & default rotation
	--Returns--: Object2D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector2,PAR_Position_Vec2,Sets the positition of the newly created 2D object
	:<T>:<F>
	<S>:*/
	pub fn New(PAR_Position_Vec2: Vector2) -> Object2D
	{
		Object2D { Position: PAR_Position_Vec2, Size: Vector2::New(1, 1), Rotation: Vector2::New(0, 0), ZIndex: 10 }
	}
	//:<S>


	/*<F>:PRT:New_PosSize
	--Description--: Creates a new instance of ['Object2D'] at default rendering ZIndex layer 10 & default rotation
	--Returns--: Object2D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector2,PAR_Position_Vec2,Sets the positition of the newly created 2D object
			Vector2,PAR_Size_Vec2,Sets the size of the newly created 2D object
	:<T>:<F>
	<S>:*/
	pub fn New_PosSize(PAR_Position_Vec2: Vector2, PAR_Size_Vec2: Vector2) -> Object2D
	{
		Object2D { Position: PAR_Position_Vec2, Size: PAR_Size_Vec2, Rotation: Vector2::New(0, 0), ZIndex: 10 }
	}
	//:<S>


	//---
	/*<F>:PRT:New_WithZIndex
	--Description--: Creates a new instance of ['Object2D'] with set ZIndex & default rotation
	--Returns--: Object2D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector2,PAR_Position_Vec2,Sets the positition of the newly created 2D object
			Vector2,PAR_Size_Vec2,Sets the size of the newly created 2D object
			u8,PAR_ZIndex_U8,Sets ZIndex rendering layer
	:<T>:<F>
	<S>:*/
	pub fn New_WithZIndex(PAR_Position_Vec2: Vector2, PAR_Size_Vec2: Vector2, PAR_ZIndex_U8: u8) -> Object2D
	{
		Object2D { Position: PAR_Position_Vec2, Size: PAR_Size_Vec2, Rotation: Vector2::New(0, 0), ZIndex: PAR_ZIndex_U8 }
	}
	//:<S>
	//---


	//---
	/*<F>:PRT:New_Full
	--Description--: Creates a new instance of ['Object2D'] with set ZIndex & default rotation
	--Returns--: Object2D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector2,PAR_Position_Vec2,Sets the positition of the newly created 2D object
			Vector2,PAR_Size_Vec2,Sets the size of the newly created 2D object
			u8,PAR_ZIndex_U8,Sets ZIndex rendering layer
	:<T>:<F>
	<S>:*/
	pub fn New_Full(PAR_Position_Vec2: Vector2, PAR_Size_Vec2: Vector2, PAR_ZIndex_U8: u8) -> Object2D
	{
		Object2D { Position: PAR_Position_Vec2, Size: PAR_Size_Vec2, Rotation: Vector2::New(0, 0), ZIndex: PAR_ZIndex_U8 }
	}
	//:<S>
	//---


	//---
	/*<F>:PRT:New_FromObject2D
	--Description--: Creates a new instance of ['Object2D'] based on passed Object2D
	--Returns--: Object2D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Object2D,PAR_Target_Object2D,The target ['Object2D'] from which should new ['Object2D'] be created
	:<T>:<F>
	<S>:*/
	pub fn New_FromObject2D(PAR_Target_Object2D: Object2D) -> Object2D
	{
		Object2D { Position: PAR_Target_Object2D.Position, Size: PAR_Target_Object2D.Size, Rotation: PAR_Target_Object2D.Rotation, ZIndex: PAR_Target_Object2D.ZIndex }
	}
	//:<S>
	//---


	/*<F>:PRT:ReLocate
	--Description--: Moves this instance to new 2D absolute position
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector2,PAR_Pos_Vec2,2D location to which this instance should be placed
	:<T>:<F>
	<S>:*/
	pub fn ReLocate(&mut self, PAR_Pos_Vec2: Vector2)
	{
		self.Position	= PAR_Pos_Vec2;
	}
	//:<S>
	//---

	/*<F>:PRT:ReSize
	--Description--: Resizes this instance to new 2D absolute position
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector2,PAR_Size_Vec2,2D Vector setting new size of this instance
	:<T>:<F>
	<S>:*/
	pub fn ReSize(&mut self, PAR_Size_Vec2: Vector2)
	{
		self.Size	= PAR_Size_Vec2;
	}
	//:<S>
	//---


	/*<F>:PRT:Move
	--Description--: Moves this instance by 2D Vector ['PAR_Pos_Vec2'] relatively to the current position of this instance
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector2,PAR_Pos_Vec2,2D Vector by which should this instance be moved
	:<T>:<F>
	<S>:*/
	pub fn Move(&mut self, PAR_Pos_Vec2: Vector2)
	{
		self.Position.X	= self.Position.X+PAR_Pos_Vec2.X;
		self.Position.Y	= self.Position.Y+PAR_Pos_Vec2.Y;
	}
	//:<S>
	//---


	/*<F>:PRT:Grow
	--Description--: ``Grows`` this 2D instance by increasing size of this instance by provided 2D Vector ['PAR_Size_Vec2']
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector2,PAR_Size_Vec2,2D Vector increasing size of this instance
	:<T>:<F>
	<S>:*/
	pub fn Grow(&mut self, PAR_Size_Vec2: Vector2)
	{
		self.Size.X	= self.Size.X+PAR_Size_Vec2.X;
		self.Size.Y	= self.Size.Y+PAR_Size_Vec2.Y;
	}
	//:<S>
	//---


	/*<F>:PRT:ReLocate_To
	--Description--: Moves this object to the position of provided Object2D ['PAR_Instance_Object2D']
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Object2D,PAR_Instance_Object2D,The object to which is this object to be moved to
			Vector2,PAR_Size_Vec2,2D Vector increasing size of this instance
	:<T>:<F>
	<S>:*/
	pub fn ReLocate_To(PAR_Instance_Object2D: Object2D, PAR_Size_Vec2: Vector2)
	{
		self.Size.X	= self.Size.X+PAR_Size_Vec2.X;
		self.Size.Y	= self.Size.Y+PAR_Size_Vec2.Y;
	}
	//:<S>
	//---
}


//CAT: Object3D implementation
//<LM>
impl Object3D
{
	/*<F>:PRT:New
	--Description--: Creates a new instance of ['Object3D'] with default size (1, 1, 1) & default rotation (0, 0, 0)
	--Returns--: Object3D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Position_Vec3,Sets the positition of the newly created 3D object
	:<T>:<F>
	<S>:*/
	pub fn New(PAR_Position_Vec3: Vector3) -> Object3D
	{
		Object3D { Position: PAR_Position_Vec3, Size: Vector3::New(1, 1, 1), Rotation: Vector3::New(0, 0, 0) }
	}
	//:<S>


	//---
	/*<F>:PRT:New_PosSize
	--Description--: Creates a new instance of ['Object3D'] & default rotation (0, 0, 0)
	--Returns--: Object3D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Position_Vec3,Sets the positition of the newly created 3D object
			Vector3,PAR_Size_Vec3,Sets the size of the newly created 3D object
	:<T>:<F>
	<S>:*/
	pub fn New_PosSize(PAR_Position_Vec3: Vector3, PAR_Size_Vec3: Vector3) -> Object3D
	{
		Object3D { Position: PAR_Position_Vec3, Size: PAR_Size_Vec3, Rotation: Vector3::New(0, 0, 0) }
	}
	//:<S>
	//---


	//---
	/*<F>:PRT:New_Full
	--Description--: Creates a new instance of ['Object3D']
	--Returns--: Object3D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Position_Vec3,Sets the positition of the newly created 3D object
			Vector3,PAR_Size_Vec3,Sets the size of the newly created 3D object
			Vector3,PAR_Rotation_Vec3,Sets the rotation of the newly created 3D object
	:<T>:<F>
	<S>:*/
	pub fn New_Full(PAR_Position_Vec3: Vector3, PAR_Size_Vec3: Vector3, PAR_Rotation_Vec3: Vector3) -> Object3D
	{
		Object3D { Position: PAR_Position_Vec3, Size: PAR_Size_Vec3, Rotation: PAR_Rotation_Vec3 }
	}
	//:<S>
	//---


	//---
	/*<F>:PRT:New_FromObject3D
	--Description--: Creates a new instance of ['Object3D'] based on passed Object3D
	--Returns--: Object3D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Object3D,PAR_Target_Object3D,The target ['Object3D'] from which should new ['Object3D'] be created
	:<T>:<F>
	<S>:*/
	pub fn New_FromObject3D(PAR_Target_Object3D: Object3D) -> Object3D
	{
		Object3D { Position: PAR_Target_Object3D.Position, Size: PAR_Target_Object3D.Size, Rotation: PAR_Target_Object3D.Rotation }
	}
	//:<S>
	//---


	/*<F>:PRT:ReLocate
	--Description--: Moves this instance to new 3D absolute position
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Pos_Vec3,3D location to which this instance should be placed
	:<T>:<F>
	<S>:*/
	pub fn ReLocate(&mut self, PAR_Pos_Vec3: Vector3)
	{
		self.Position	= PAR_Pos_Vec3;
	}
	//:<S>
	//---

	/*<F>:PRT:ReSize
	--Description--: Resizes this instance to new 3D absolute position
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Size_Vec3,3D Vector setting new size of this instance
	:<T>:<F>
	<S>:*/
	pub fn ReSize(&mut self, PAR_Size_Vec3: Vector3)
	{
		self.Size	= PAR_Size_Vec3;
	}
	//:<S>
	//---


	/*<F>:PRT:Move
	--Description--: Moves this instance by 3D Vector ['PAR_Pos_Vec3'] relatively to the current position of this instance
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Pos_Vec3,3D Vector by which should this instance be moved
	:<T>:<F>
	<S>:*/
	pub fn Move(&mut self, PAR_Pos_Vec3: Vector3)
	{
		self.Position.X	= self.Position.X+PAR_Pos_Vec3.X;
		self.Position.Y	= self.Position.Y+PAR_Pos_Vec3.Y;
		self.Position.Z	= self.Position.Z+PAR_Pos_Vec3.Z;
	}
	//:<S>
	//---


	/*<F>:PRT:Grow
	--Description--: ``Grows`` this 3D instance by increasing size of this instance by provided 3D Vector ['PAR_Size_Vec3']
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Size_Vec3,3D Vector increasing size of this instance
	:<T>:<F>
	<S>:*/
	pub fn Grow(&mut self, PAR_Size_Vec3: Vector3)
	{
		self.Size.X	= self.Size.X+PAR_Size_Vec3.X;
		self.Size.Y	= self.Size.Y+PAR_Size_Vec3.Y;
		self.Size.Z	= self.Size.Z+PAR_Size_Vec3.Z;
	}
	//:<S>
	//---
}


//CAT: Primitive3D implementation
//<LM>
impl Primitive3D
{
	/*<F>:PRT:New
	--Description--: Creates a new instance of ['Primitive3D'] with defaultly set Base ['Object3D'] & default color ``(white)`` 255, 255, 255, 0
	--Returns--: Primitive3D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Pos_Vec3,3D location to which new instance should be placed
			Primitive3DShape,PAR_Shape_E,3D Shape of the new instance
	:<T>:<F>
	<S>:*/
	pub fn New(PAR_Pos_Vec3: Vector3, PAR_Shape_E: Primitive3DShape) -> Primitive3D
	{
		Primitive3D { Base: Object3D::New(PAR_Pos_Vec3), Shape: PAR_Shape_E, Color: Color::New(255, 255, 255, 0) }
	}
	//:<S>
	//---


	/*<F>:PRT:New_Base
	--Description--: Creates a new instance of ['Primitive3D'] settings its Positin, Size & Rotation to those from passed ['Object3D'] base & default color ``(white)`` 255, 255, 255, 0
	--Returns--: Primitive3D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Object3D,PAR_Base_Object3D,3D Object which will be used as base
			Primitive3DShape,PAR_Shape_E,3D Shape of the new instance
	:<T>:<F>
	<S>:*/
	pub fn New_Base(PAR_Base_Object3D: Object3D, PAR_Shape_E: Primitive3DShape) -> Primitive3D
	{
		Primitive3D { Base: PAR_Base_Object3D, Shape: PAR_Shape_E, Color: Color::New(255, 255, 255, 0) }
	}
	//:<S>
	//---


	/*<F>:PRT:ReShape
	--Description--: Changes shape of this instance
	--Returns--:nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			Primitive3DShape,PAR_Shape_E,New shape for this instance
	:<T>:<F>
	<S>:*/
	pub fn ReShape(&mut self, PAR_Shape_E: Primitive3DShape)
	{
		self.Shape	= PAR_Shape_E;
	}
	//:<S>
	//---
}
//---


// Implementations !REM
//----------------------------
// SET: Trigger Functions

pub fn Yes() -> String
{
	"Yes".to_string()
}

// Trigger Functions !REM
//----------------------------
// SET: AUTORUN Functions

fn main()
{

}