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


pub enum Primitive2DShape
{
	Rectangle,
	Circle
}


pub enum Primitive3DShape
{
	Cube,
	Ball,
	Cilinder
}

//----------------------------

pub struct Color
{
	R: f32,
	G: f32,
	B: f32,
	A: f32
}

pub struct Vector2
{
	X: f32,
	Y: f32
}


pub struct Vector3
{
	X: f32,
	Y: f32,
	Z: f32
}


pub struct Point2D
{
	Position: Vector2
}


pub struct Point3D
{
	Position: Vector3
}


pub struct Object2D
{
	Position: Vector2,
	Size: Vector2,
}

pub struct Primitive2D
{
	pub Base: Object2D,
	Shape: Primitive2DShape
}

//SET: Object3D
/*--Object3D-- is a base for all 3D ``based`` objects like primitive, mesh, etc
It composes of:
<T>:Name,Type,Usage
Position,Vector3,The object position in 3D space
Size, Vector3,The object's size in 3D space
:<T>
<S>:*/
pub struct Object3D
{
	Position: Vector3,
	Size: Vector3,
}
//:<S>


pub struct Primitive3D
{
	pub Base: Object3D,
	Shape: Primitive3DShape
}

// ENUMs / STRUCs !REM
//----------------------------
// SET: Implementations

//SET: Vector2 implementation
//<LM>
impl Vector2
{
	/*<F>:CAT:New
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
	/*<F>:CAT:New_FromF32
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

//SET: Vector3 implementation
//<LM>
impl Vector3
{
	/*<F>:CAT:New
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
	/*<F>:CAT:New_FromF32
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


//SET: Object3D implementation
//<LM>
impl Object3D
{
	/*<F>:CAT:New
	--Description--: Creates a new instance of ['Vector3']
	--Returns--: Vector3
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Position_Vec3,Sets the positition of the newly created 3D object
			Vector3,PAR_Size_Vec3,Sets the size of the newly created 3D object
	:<T>:<F>
	<S>:*/
	pub fn New(PAR_Position_Vec3: Vector3, PAR_Size_Vec3: Vector3) -> Object3D
	{
		Object3D { Position: PAR_Position_Vec3, Size: PAR_Size_Vec3 }
	}
	//:<S>
	//---


	/*<F>:CAT:ReLocate
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

	/*<F>:CAT:ReSize
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


	/*<F>:CAT:Move
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


	/*<F>:CAT:Grow
	--Description--: ``Grows`` this 3D instance by increasing size of this instance by provided 3D Vector ['PAR_Pos_Vec3']
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


//SET: Primitive3D implementation
//<LM>
impl Primitive3D
{
	/*<F>:CAT:New
	--Description--: Creates a new instance of ['Primitive3D']
	--Returns--: Primitive3D
	--Parameters--:
	<T>:	DataType,Name,Usage
			Vector3,PAR_Pos_Vec3,3D location to which new instance should be placed
			Vector3,PAR_Size_Vec3,3D size of new instance
			Primitive3DShape,PAR_Shape_E,3D Shape of the new instance
	:<T>:<F>
	<S>:*/
	pub fn New(PAR_Pos_Vec3: Vector3, PAR_Size_Vec3: Vector3, PAR_Shape_E: Primitive3DShape) -> Primitive3D
	{
		Primitive3D { Base: Object3D::New(PAR_Pos_Vec3, PAR_Size_Vec3), Shape: PAR_Shape_E }
	}
	//:<S>
	//---

	/*<F>:CAT:ReShape
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
	let mut box3d: Primitive3D = Primitive3D::New(Vector3::New(1, 5, 6), Vector3::New(5, 5, 5), Primitive3DShape::Cilinder);

	box3d.Base.ReSize(Vector3::New(5, 5, 5));

	"Yes".to_string()
}

// Trigger Functions !REM
//----------------------------
// SET: AUTORUN Functions

fn main()
{

}