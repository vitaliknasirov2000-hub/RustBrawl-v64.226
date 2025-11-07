pub struct Messaging;

impl Messaging 
{
    pub fn writeHeader(a1: &mut [u8], a2: i32, a3: u32, a4: i32) 
    {
        a1[0] = ((a2 >> 8) & 0xFF) as u8;
        a1[1] = (a2 & 0xFF) as u8;

        a1[2] = ((a3 >> 16) & 0xFF) as u8;
        a1[3] = ((a3 >> 8) & 0xFF) as u8;
        a1[4] = (a3 & 0xFF) as u8;

        a1[5] = ((a4 >> 8) & 0xFF) as u8;
        a1[6] = (a4 & 0xFF) as u8;
    }

    pub fn readHeader(a1: &[u8]) -> (i32, u32, i32) 
    {
        let v1 = ((a1[0] as i32) << 8) | (a1[1] as i32);
        let v2 = ((a1[2] as u32) << 16) | ((a1[3] as u32) << 8) | (a1[4] as u32);
        let v3 = ((a1[5] as i32) << 8) | (a1[6] as i32);

        (v1, v2, v3)
    }
}
