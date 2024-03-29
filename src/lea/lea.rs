const KEY_CONST: [u32; 8] = [0xc3efe9db, 0x44626b02, 0x79e27c8a, 0x78df30ec, 0x715ea49e, 0xc785da0a, 0xe04ef22a, 0xe5c40957 ];
// const ROUND_KEY: [u32; 192] = [0; 192];
const MASTER_KEY: [u32; 4] = [0x3C2D1E0F, 0x78695A4B, 0xB4A59687, 0xF0E1D2C3]; //128-bit
const PLAIN_TEXT: [u32; 4] = [0x13121110, 0x17161514, 0x1B1A1918, 0x1F1E1D1C]; //128-bit

fn rol(x: u32, n: u8) -> u32 {
    x.rotate_left(u32::from(n))
}

fn ror(x: u32, n: u8) -> u32 {
    x.rotate_right(u32::from(n))
}

fn enc_round_key_gen(mk: &[u32; 4], rk: &mut [u32; 192]) {
    let mut temp: [u32; 4] = [0; 4];
    println!("enc_Round_key_gen start : ");
    println!("ㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡ ");
    // print!("SecretKey in Round_key_gen : ");
    for i in 0..4 {
        temp[i] = mk[i];
    
    //     print!("0x{:08X} ", temp[i]);
    }
    for i in 0..24 {
        temp[0] = rol(temp[0].wrapping_add(rol(KEY_CONST[i % 4], i as u8)), 1);
        temp[1] = rol(temp[1].wrapping_add(rol(KEY_CONST[i % 4], (i + 1) as u8)), 3);
        temp[2] = rol(temp[2].wrapping_add(rol(KEY_CONST[i % 4], (i + 2) as u8)), 6);
        temp[3] = rol(temp[3].wrapping_add(rol(KEY_CONST[i % 4], (i + 3) as u8)), 11);
        rk[i * 6] = temp[0];
        rk[i * 6 + 1] = temp[1];
        rk[i * 6 + 2] = temp[2];
        rk[i * 6 + 3] = temp[1];
        rk[i * 6 + 4] = temp[3];
        rk[i * 6 + 5] = temp[1];
    }
    println!();
}
fn dec_round_key_gen(mk: &[u32; 4], rk: &mut [u32; 192]) {
    let mut temp: [u32; 4] = [0; 4];
    println!("dec_Round_key_gen start : ");
    println!("ㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡ ");
    // print!("SecretKey in Round_key_gen : ");
    for i in 0..4 {
        temp[i] = mk[i];
    
    //     print!("0x{:08X} ", temp[i]);
    }
    for i in 0..24 {
        temp[0] = rol(temp[0].wrapping_add(rol(KEY_CONST[i % 4], i as u8)), 1);
        temp[1] = rol(temp[1].wrapping_add(rol(KEY_CONST[i % 4], (i + 1) as u8)), 3);
        temp[2] = rol(temp[2].wrapping_add(rol(KEY_CONST[i % 4], (i + 2) as u8)), 6);
        temp[3] = rol(temp[3].wrapping_add(rol(KEY_CONST[i % 4], (i + 3) as u8)), 11);
        rk[138 -i * 6] = temp[0];
        rk[139 -i * 6] = temp[1];
        rk[140 -i * 6] = temp[2];
        rk[141 -i * 6] = temp[1];
        rk[142 -i * 6] = temp[3];
        rk[143 -i * 6] = temp[1];
    }
    // for i in 0..144{
    //     println!("rk : 0x{:08X}", rk[i]);
    // }


}

fn round_key_gen_256(mk: &[u32; 8], rk: &mut [u32; 192]) {
        let mut temp: [u32; 8] = [0; 8];

        for i in 0..8 {
            temp[i] = mk[i];
            print!("T : {}", temp[i]);
        }
    println!();

        for i in 0..32 {
            temp[(6 * i) % 8] = rol(temp[0].wrapping_add(rol(KEY_CONST[i % 8], i as u8)), 1);
            temp[(6 * i + 1) % 8] = rol(temp[1].wrapping_add(rol(KEY_CONST[i % 8], (i+1) as u8)), 3);
            temp[(6 * i + 2) % 8] = rol(temp[2].wrapping_add(rol(KEY_CONST[i % 8], (i+2) as u8)), 6);
            temp[(6 * i + 3) % 8] = rol(temp[3].wrapping_add(rol(KEY_CONST[i % 8], (i+3) as u8)), 11);
            temp[(6 * i + 4) % 8] = rol(temp[4].wrapping_add(rol(KEY_CONST[i % 8], (i+4) as u8)), 13);
            temp[(6 * i + 5) % 8] = rol(temp[5].wrapping_add(rol(KEY_CONST[i % 8], (i+5) as u8)), 17);
            rk[i * 6] = temp[(6 * i) % 8];
            rk[i * 6 + 1] = temp[(6 * i + 1) % 8];
            rk[i * 6 + 2] = temp[(6 * i + 2) % 8];
            rk[i * 6 + 3] = temp[(6 * i + 3) % 8];
            rk[i * 6 + 4] = temp[(6 * i + 4) % 8];
            rk[i * 6 + 5] = temp[(6 * i + 5) % 8];
        }
}



fn enc(x: &mut [u32; 4], rk: &[u32; 192]) ->[u32;4] {
    println!("Enc start : ");
    println!("ㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡ ");
    let mut temp;


    for i in 0..24 {

        let temp1 = x[0] ^ rk[i * 6];
        let temp2 = x[1] ^ rk[i * 6 + 2];
        let temp3 = x[2] ^ rk[i * 6 + 4];
        temp = x[0];
        x[0] = rol(temp1.wrapping_add(x[1] ^ rk[(i * 6) + 1]), 9);
        x[1] = ror(temp2.wrapping_add(x[2] ^ rk[(i * 6) + 3]), 5);
        x[2] = ror(temp3.wrapping_add(x[3] ^ rk[(i * 6) + 5]), 3);
        x[3] = temp;
        // println!("X{} : ", i+1);
        // for j in 0..4 {
        //     print!("0x{:08X} ", x[j]);
        // }
        //
        //
        // print!("\n");
    }
*x
}

fn dec(x: &mut [u32; 4], rk: &[u32; 192])->[u32;4]  {
    println!("Dec start : ");
    println!("ㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡㅡ ");

    // println!("초기 값 : ",);
    // for j in 0..4 {
    //     print!("0x{:08X} ", x[j]);
    // }
    // println!();
    // println!("rk[0] 0x{:08X}", rk[0] );
    // println!("rk[143] 0x{:08X}", rk[143] );
    for i in 0..24 {
        let temp0 = x[0];
        let temp1 = x[1];
        let temp2 = x[2];
        let temp3 = x[3];
        x[0] = temp3;
        x[1] = (ror(temp0,9).wrapping_sub(x[0]^rk[i*6])) ^rk[i*6 + 1];
        x[2] = (rol(temp1,5).wrapping_sub(x[1]^rk[i*6 + 2 ])) ^rk[i*6 + 3];
        x[3] = (rol(temp2,3).wrapping_sub(x[2]^rk[i*6 + 4 ])) ^rk[i*6 + 5];
        // println!("X{} : ", i+1);
        // for j in 0..4 {
        //     print!("0x{:08X} ", x[j]);
        // }
        // print!("\n");
    }
    *x
}

pub fn LEA_TEST() {
    let mut plain_text: [u32; 4] = [0x13121110, 0x17161514, 0x1B1A1918, 0x1F1E1D1C]; //128-bit
    let mut enc_round_key: [u32; 192] = [0; 192];
    let mut dec_round_key: [u32; 192] = [0; 192];


    println!("plain text u32");
    for i in 0..4{
        print!("0x{:08X} ", PLAIN_TEXT[i]);
    }
    println!("");

    enc_round_key_gen(&MASTER_KEY, &mut enc_round_key);
    let mut encrypted_text = enc(&mut plain_text, &mut enc_round_key);

    println!("");
    print!("encrypted_text : ");
    for value in encrypted_text.iter() {
        print!("0x{:08X} ", value);
    }
    println!();
    dec_round_key_gen(&MASTER_KEY, &mut dec_round_key);
    let mut decrypted_text = dec(&mut encrypted_text, &mut dec_round_key);

    println!("");
    print!("decrypted_text :");
    for value in decrypted_text.iter() {
        print!(" 0x{:08X} ", value);
    }
    println!();
}

