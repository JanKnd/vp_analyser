use std::fs::DirEntry;

#[derive(Debug)]
pub struct Vertretungsplaene{
    pub montag: String,
    pub dienstag: String,
    pub mittwoch: String,
    pub donnerstag: String,
    pub freitag: String
}

impl Vertretungsplaene{
    pub fn new() -> Vertretungsplaene{
        Vertretungsplaene{
            montag: pdf_extract::extract_text("VP/Montag/Montag S.pdf").unwrap().to_string(),
            dienstag: pdf_extract::extract_text("VP/Dienstag/Dienstag S.pdf").unwrap().to_string(),
            mittwoch: pdf_extract::extract_text("VP/Mittwoch/Mittwoch S.pdf").unwrap().to_string(),
            donnerstag:pdf_extract::extract_text("VP/Donnerstag/Donnerstag S.pdf").unwrap().to_string(),
            freitag:pdf_extract::extract_text("VP/Donnerstag/Donnerstag S.pdf").unwrap().to_string()
        }
    }

    pub fn all(&self) -> String {
        format!("Montag:{}\
                Dienstag:{}\
                Mittwoch{}\
                Donnerstag:{}\
                Freitag:{}",self.montag,self.dienstag,self.mittwoch,self.donnerstag,self.freitag)
    }

    pub fn to_tuple(self) -> (String,String,String,String,String){
        (self.montag,self.dienstag,self.mittwoch,self.donnerstag,self.freitag)
    }

}
