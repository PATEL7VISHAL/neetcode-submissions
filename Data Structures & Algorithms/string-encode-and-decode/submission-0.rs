impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let len = strs.len();
        let mut counts: Vec<u8> = Vec::with_capacity(len);
        let mut encoded_str = String::with_capacity(len + 1);
        encoded_str.push(len as u8 as char);
        (0..len).for_each(|e| {
            encoded_str.push(0 as char);
        });

        for s in strs.into_iter() {
            encoded_str.push_str(&s);
            counts.push(s.len() as u8);
        }

        let buytes = unsafe { encoded_str.as_bytes_mut() };
        counts.into_iter().enumerate().for_each(|(i, count)| {
            buytes[i + 1] = count as u8;
        });

        return encoded_str;
    }

    pub fn decode(s: String) -> Vec<String> {
        let m = s.as_bytes();
        let len = m[0] as usize;
        let mut ans: Vec<String> = Vec::with_capacity(len);

        let counter_ref = &m[1..=len];
        let mut current = len+1;
        for count in counter_ref {
            let str_len = *count as usize;
            let str = String::from_utf8_lossy(&m[current..(current + str_len)]).to_string();
            ans.push(str);
            current += str_len;
        }
        return ans;
    }
}
