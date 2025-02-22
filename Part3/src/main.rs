fn generate_sql(username: String, password: String) -> String{
    let denylist: [char;2] = ['=','\''];
    let allowlist: [char;69] = [
        '~','!','@','$','%','^','&',
        '1','2','3','4','5','6','7','8','9','0',
        'a','b','c','d','e','f','g','h','i','j','k','l','m',
        'n','o','p','q','r','s','t','u','v','w','x','y','z',
        'A','B','C','D','E','F','G','H','I','J','K','L','M',
        'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut is_valid: bool = true;
    if username.contains(denylist) {is_valid = false;}
    if (!password.contains(&allowlist[..=6]) || !password.contains(&allowlist[7..=16]) ||
        !password.contains(&allowlist[17..=42]) || !password.contains(&allowlist[43..=68])) || password.len() < 8 {
            is_valid = false;
        }
    for i in password.chars() {
        if !allowlist.contains(&i) {
            is_valid = false;
            break;
        }
    }
    if !is_valid {return "Error: Invalid Credentials".to_string()};
    return format!("SELECT *\nFROM accounts\nWHERE userid = {} AND pswd = {}",username,password);
}

#[cfg(test)]
mod tests {
    use super::*;
    const valid_username: [&str;10] = ["User_Name","un","usern@me","xX_USER_NAME_Xx","UsErNaMe.../---/...","ZYXWV!!USER_____NAME!!DCBA",
                                        "()()()!!!!!4321eman_resu^*^#*@","-TTTT-useRnamE","UzerNaaaaaame","NamserUe"];
    const valid_password: [&str;10] = ["ThisIs@ValidPassw0rd!","So1sThis!","$afe~P@ssw0rd","ValidCred3nt1@l","L3tM3InPLEASE!!!!",
                                        "!!!!T0P^SeCRET!!!!","Acce22~granteD~!","Va1id&Va1id!","Different&SafePassw0rd","2ManyPASSWORDS!"];
    
    #[test]
    fn test_valid_credentials() {
        for i in 0..10 {
            assert_eq!(generate_sql(valid_username[i].to_string(),valid_password[i].to_string()),
            (format!("SELECT *\nFROM accounts\nWHERE userid = {} AND pswd = {}",valid_username[i].to_string(),valid_password[i].to_string())),
            "ERROR: INVALID CREDENTIAL!");
        }
    }
    const invalid_username: [&str;10] = ["u=n","user=name","\'\'username","==USERNAME==","bad_username\\'==",
                                        "\\'===worse_username===\\'","=undername","=under_where?","under=here","wow\\'haha"];
    const invalid_password: [&str;10] = ["invalid password","bad password","bad password?","yes, bad password.","h0w about this?",
                                        "(()))))STILL BAD!)@(!@)(@!)(","NoIdeaNow...","tictockTICTOCK!!@!#*$*$*","ha",".../---/..."];

    #[test]
    fn test_invalid_credentials_username() {
        for i in 0..10 {
            assert_eq!(generate_sql(invalid_username[i].to_string(),valid_password[i].to_string()),
            "Error: Invalid Credentials","ERROR: VALID CREDENTIAL!");
        }
    }
    #[test]
    fn test_invalid_credentials_password() {
        for i in 0..10 {
            assert_eq!(generate_sql(valid_username[i].to_string(),invalid_password[i].to_string()),
            "Error: Invalid Credentials","ERROR: VALID CREDENTIAL!");
        }
    }
}

fn main() {
    println!("{}",generate_sql("testuser".to_string(),"testpass1!".to_string()));
}
