fn main() {
    let string1 = "GetSkillsFast";
    assert_eq!("GetSkillsFast", string1);

    let string2 = "GetSkills
        Fast";
    // Remember to use the right amount of spaces for the tab
    assert_eq!("GetSkills\n        Fast", string2);

    let string3 = "GetSkills\
        Fast";
    assert_eq!("GetSkillsFast", string3);
}
