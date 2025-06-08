// SET: Script Info
/*<I>:
© IVN - Iterik Viscela Nova, © Matěj Zahradník, © TERA_Ventl
Development Standard:	IVN_ATK-ATL_Provisions:TERA:762:XR
------------------------------
Project leader:		-
Supervisor:			Matěj Zahradník [The_Universality] - © Matěj Zahradník
Scripted by: 		Matěj Zahradník [The_Universality] - © Matěj Zahradník
------------------------------
Created at:			15:09 | 03.06.2025 [D.M.Y]
------------------------------
Script type:		Definition Script (DS)
Script purpose:		Handles CLI program system & Arguments
:<I>*/

// Script Info !REM
//----------------------------
// SET: Copliler messages

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

// Copliler messages !REM
//----------------------------
// SET: Package and Import statements

//<S>:

//:<S>

// Package and Import statements !REM
//----------------------------
// SET: ENUMs / STRUCs

enum Primitive2DShape
{
	Rectangle,
	Circle
}

enum Primitive3DShape
{
	Cube,
	Ball,
	Cilinder
}

struct Vector2
{
	X: isize,
	Y: isize
}
struct Vector3
{
	X: isize,
	Y: isize,
	Z: isize
}

struct Point2D
{
	Position: Vector2
}
struct Point3D
{
	Position: Vector3
}

struct Primitive2D
{
	Position: Vector2,
	Size: Vector2,
	Shape: Primitive2DShape
}
struct Primitive2D
{
	Position: Vector3,
	Size: Vector3,
	Shape: Primitive3DShape
}