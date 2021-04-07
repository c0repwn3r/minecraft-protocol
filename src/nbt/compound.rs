use super::*;
use std::collections::HashMap;

#[inline]
pub fn parse_compound(
    mut input: &mut [u8],
) -> Result<(HashMap<&str, NbtTag>, &mut [u8]), &'static str> {
    let mut content = HashMap::new();

    loop {
        if input.first() == Some(&0) {
            input = &mut input[1..];
            break;
        }
        if input.len() < 3 {
            return Err("A tag in a compound should be introduced by three bytes.");
        }
        let (tag_id, len): (u8, u16) =
            unsafe { (*input.get_unchecked(0), read_u16(&mut input[1..])) };

        let len = len as usize;
        let new_input = &mut input[3..];
        let (bytes, new_input) = new_input.split_at_mut(len);
        let name = std::str::from_utf8(bytes)
            .map_err(|_| "A tag name should contain valid utf8 characters.")?;
        let (tag, new_input) = parse_nbt_tag(new_input, tag_id)?;
        input = new_input;
        content.insert(name, tag);
    }

    Ok((content, input))
}

pub fn parse_root_compound(
    mut input: &mut [u8],
) -> Result<((&str, HashMap<&str, NbtTag>), &mut [u8]), &'static str> {
    if input.first() != Some(&10) {
        return Err("The root compound tag should start with the compound ID (10).");
    }
    input = &mut input[1..];
    if input.len() < 2 {
        return Err("A root compound tag should contain two bytes.");
    }
    let len: u16 = unsafe { read_u16(input) };
    let len = len as usize;
    input = &mut input[2..];
    let (bytes, new_input) = input.split_at_mut(len);
    let name = std::str::from_utf8(bytes)
        .map_err(|_| "A compound tag name should contain valid utf8 characters.")?;
    input = new_input;

    let (content, input) = parse_compound(input)?;

    Ok(((name, content), input))
}

pub fn parse_root_compound_complete(
    mut input: &mut [u8],
) -> Result<(&str, HashMap<&str, NbtTag>), &'static str> {
    let (value, input) = parse_root_compound(input)?;

    if !input.is_empty() {
        return Err("There should be no data after a root compound.");
    }

    Ok(value)
}