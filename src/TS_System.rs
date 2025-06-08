// SET: Script Info
/*<I>:
© IVN - Iterik Viscela Nova, Matěj Zahradník, Open Source
Development Standard:	IVN_ATK-ATL_Provisions:TERA:762:XR
------------------------------
Project leader:		-
Supervisor:			Matěj Zahradník [The_Universality] - © Matěj Zahradník
Scripted by: 		Matěj Zahradník [The_Universality] - © Matěj Zahradník
------------------------------
Created at:			15:09 | 03.06.2025 [D.M.Y]
------------------------------
Script type:		Trigger Script (TS)
Script purpose:		Handles CLI program system & Arguments
:<I>*/

// Script Info !REM
//----------------------------
// SET: Compliler messages

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

// Compliler messages !REM
//----------------------------
// SET: Package and Import statements

//<S>:
mod main;
mod Modules;

use {std::{self, env, fs::File, io::Read, ops::Add, path::{Path, PathBuf}, process::exit}};
//:<S>

// Package and Import statements !REM
//----------------------------
// SET: ENUMs / STRUCs



// ENUMs / STRUCs !REM
//----------------------------
// SET: Program Variables

// CLI Program version | ['CV_Str_Version']<LM>
const CV_Str_Version: &str	= "0.00.00.U | 0.00.001.D";

// Program Variables !REM
//----------------------------
// SET: Trigger Functions


/*<F>:CAT: F_ShowHelp_RNil
	--Description--: Shows help message
	--Returns--: nil
	--Parameters--:
	<T>:	DataType,Name,Usage
			&str,PAR_HelpFile_Str,If empty; shows general help; otherwise show help from selected file
	:<T>:<F>
<LM>*/
fn F_ShowHelp_RNil(PAR_HelpFile_Str: &str)
{
	// Path of this program - needed for ['V_Path_EXEDir'] | ['V_Path_EXEPath']<S>:
	let V_Path_EXEPath: PathBuf		= match env::current_exe()
	{
		Ok(RESULT_SUCCESS)			=> 
		{	// This is a RETURN statement that REJECTS the RETURN keyword
			RESULT_SUCCESS
		},
		Err(RESULT_ERROR)			=>
		{
			println!("\n***********************\n[ERROR]:\tFailed to obtain program path! [EXIT CODE: 20]\n\t{}",RESULT_ERROR);

			// Program EXIT - __--20 - Failed to obtain program path / get parent path--__<LM>
			exit(20);
		}
	};
	//:<S>

	// Directory in which this program is located - ``used to access program related files`` | ['V_Path_EXEDir']<S>:
	let V_Path_EXEDir: &Path		= match V_Path_EXEPath.parent()
	{
		Some(RESULT_SUCCESS)		=>
		{
			RESULT_SUCCESS
		},
		None						=>
		{
			println!("\n***********************\n[ERROR]:\tFailed to obtain parent path! [EXIT CODE: 20]");

			// Program EXIT - __--20 - Failed to obtain program path / get parent path--__<LM>
			exit(20);
		}
	};
	//:<S>

	// Stores relative path & name of help file | ['V_Str_FileName']
	let V_Str_FileName: &str = &("FLD_Resources/".to_string().add(&PAR_HelpFile_Str).add(".IVN_Docu"));

	// Stores path to related help file | ['V_Path_FilePath']<LM>
	let V_Path_FilePath: PathBuf	= V_Path_EXEDir.join(if(PAR_HelpFile_Str == ""){println!("Program GENERAL meaning:\n"); "FLD_Resources/Help.IVN_Docu"} else {println!("Program SPECIFIC help: {}\n",PAR_HelpFile_Str); V_Str_FileName});


	// Checks if file exists<S>:
	if(Path::exists(V_Path_FilePath.as_path())	== false)
	{
		println!("\n***********************\n[ERROR]:\tHelp file failed to load/not present. [EXIT CODE: 21]\n{}\n", V_Path_EXEPath.as_path().to_string_lossy());

		// Program EXIT - __--21 - Help file failed to load/not present--__<LM>
		exit(21);
	}
	//:<S>

	// Attempts to obain file containing Exit Codes | ['V_File_ExitCodes']<LM>
	let mut V_File_ExitCodes: File	= File::open(V_Path_FilePath).expect("!ALERT! File obtain/read failed");

	// A string to which will be stored contents of ExitCodes files | ['V_Str_ReadData']<LM>
	let mut V_Str_ReadData: String	= String::new();
	// Used as a filtering for ExitCodes file to not show anything outside table of this file | ['V_Bool_IsInCodes']<LM>
	let mut V_Bool_IsInCodes: bool	= false;

	// Reads ExitCodes file and stores its content as string into ['V_Str_ReadData']<LM>
	V_File_ExitCodes.read_to_string(&mut V_Str_ReadData).expect("msg");


	// For each line of ExitCodes file contents,<S>:
	for I_Str in V_Str_ReadData.split('\n')
	{
		// If line contains table & highligh end tag, cease printing ['I_Str']
		if(I_Str.contains(":<T>:<F>")		== true)
		{
			V_Bool_IsInCodes				= false;
			println!("-------------------------------------\n");
		}
		// If line contains table & highligh start tag, start printing ['I_Str']
		else if(I_Str.contains("<F>:<T>:")	== true)
		{
			V_Bool_IsInCodes				= true;

			let V_Str_EditedStr: String		= I_Str.replace("<F>:<T>:", "");

			// Stored fist element of help file line | ['V_StrSplit_']<S>:
			let V_StrSplit_: &str			= match V_Str_EditedStr.split(",").nth(0)
			{
				Some(RESULT_SUCCESS)		=>
				{
					RESULT_SUCCESS
				}
				None						=>
				{
					"<:ISSUE:>: WITH READING/SPLITTING"
				}
			};
			//:<S>

			if(V_StrSplit_.len()			> 7)
			{
				println!("{}", V_Str_EditedStr.replace(",", "\t| "));
			}
			else
			{
				println!("{}", V_Str_EditedStr.replace(",", "\t\t| "));
			}

			println!("-------------------------------------");
		}
		// If ['V_Bool_IsInCodes'] is --true--, print ['I_Str']
		else if(V_Bool_IsInCodes			== true)
		{
			// Stored fist element of help file line | ['V_StrSplit_']<S>:
			let V_StrSplit_: &str			= match I_Str.split(",").nth(0)
			{
				Some(RESULT_SUCCESS)	=>
				{
					RESULT_SUCCESS
				}
				None						=>
				{
					"<:ISSUE:>: WITH READING/SPLITTING"
				}
			};
			//:<S>

			if(V_StrSplit_.len()			> 7)
			{
				println!("{}", I_Str.replace(",", "\t| "));
			}
			else
			{
				println!("{}", I_Str.replace(",", "\t\t| "));
			}

		}
	}
	//:<S>

	println!("\n***********************\n(NOTE):\tProgram showed help message. [EXIT CODE: 2]\n***********************\n");

	// Program EXIT - __--2 - Program showed help message--__<LM>
	exit(2);
}
//---


// Trigger Functions !REM
//----------------------------
// SET: AUTORUN Functions

/*<F>:CAT:main
	--Description--: Program ENTRY; handles inputted arguments
	--Returns--:nil
	--Parameters--: nil
:<F>*/
fn main()
{
	// Stores arguments passed in | ['PAR_Arguments_StrVec']<LM>
	let PAR_Arguments_StrVec: Vec<String>	= env::args().collect();


	// Introduction message <S>:
	print!("+---+  +---+              +---+  +---+    +---+\n");
	print!("|   |   \\   \\            /   /   |   |\\   |   |\n");
	print!("|   |    \\   \\          /   /    |   | \\  |   |\n");
	print!("|   |     \\   \\        /   /     |   |  \\ |   |\n");
	print!("|   |      \\   \\      /   /      |   |   \\|   |\n");
	print!("|   |       \\   \\    /   /       |   |\\   \\   |\n");
	print!("|   |        \\   \\  /   /        |   | \\   \\  |\n");
	print!("|   |         \\   \\/   /         |   |  \\   \\ |\n");
	print!("|   |          \\      /          |   |   \\   \\|\n");
	print!("+---+           +----+           +---+    +---+\n\n***********************\n");
	print!("Engine System Service\n\n(C) Open Source.\nVersion: {}\n\n| TERA_Ventl - Engine System Service |\n\n***********************\n",{CV_Str_Version});
	//:<S>

	// Current executing dirrectory | ['V_Path_CurExeDir']<S>:
	let V_Path_CurExeDir: PathBuf			= match env::current_dir()
	{
		Ok(RESULT_SUCCESS)			=> 
		{
			RESULT_SUCCESS
		},
		Err(RESULT_ERROR)			=>
		{
			println!("\n***********************\n[ERROR]:\tFailed to obtain program path! [EXIT CODE: 20]\n\t{}",RESULT_ERROR);

			// Program EXIT - __--20 - Failed to obtain program path / get parent path--__<LM>
			exit(20);
		}
	};
	//:<S>
	println!("(NOTE):\tCurrent dirrectory: {}\n\tCurrent operating system: {}\n\n", V_Path_CurExeDir.to_str().expect("[ERROR]\tFail with CurExeDir"), env::consts::OS);

	// If there's no arguments passed to program;<S>:
	if(PAR_Arguments_StrVec.len()					< 2)
	{
		// Provice hint message
		println!("\n***********************\n(NOTE):\tNo arguments give; exiting program [EXIT CODE: 1]\n\tType \"-h\"/\"-help\" to see program options.");

		// Program EXIT - __--1 - Program received no arguments--__<LM>
		exit(1);
	}
	//:<S>

	// Shows help message via ['F_ShowHelp_RNil'] function, if first argument is "-h" or "-help"<S>:
	if(PAR_Arguments_StrVec[1].to_lowercase()		== "-h" || PAR_Arguments_StrVec[1].to_lowercase()	== "-help")
	{
		if(PAR_Arguments_StrVec.len() > 2)
		{
			F_ShowHelp_RNil(&PAR_Arguments_StrVec[2]);
		}
		else
		{
			F_ShowHelp_RNil("");
		}
		
	}
	//:<S>

	// Loops via rest of the arguments<LM>
	for I_Argument in PAR_Arguments_StrVec
	{
		// If ``render`` is found as argument, launch debuging rendering window<S>:
		if(I_Argument	== "render")
		{
			main::Renderer();
		}
		//:<S>
	}
}
//---