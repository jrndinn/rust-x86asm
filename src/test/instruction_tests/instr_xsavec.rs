use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsavec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectDisplaced(BP, 82, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 102, 82], OperandSize::Word)
}

#[test]
fn xsavec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1061786495, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 36, 213, 127, 147, 73, 63], OperandSize::Dword)
}

#[test]
fn xsavec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(Indirect(RBX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 35], OperandSize::Qword)
}
