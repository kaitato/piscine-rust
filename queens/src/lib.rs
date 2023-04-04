fn main() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? => {}",
        white_queen.can_attack(&black_queen)
    );

    let white_queen = Queen::new(ChessPosition::new(1, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? => {}",
        white_queen.can_attack(&black_queen)
    );
}

// level 3: queens (karyun.cheung)

#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
		if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
			Some(Self{
				rank,
				file,
			})
		} else {
			None
		}

    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
		Queen {
			position,
		}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
		if self.position.rank == other.position.rank {
			return true;
		}
		if self.position.file == other.position.file {
			return true;
		}
		let rank_dif = if self.position.rank > other.position.rank {
			self.position.rank - other.position.rank
		} else if self.position.rank < other.position.rank {
			other.position.rank - self.position.rank
		} else {
			self.position.rank
		};
		let file_dif = if self.position.file > other.position.file {
			self.position.file - other.position.file
		} else if self.position.file < other.position.file {
			other.position.file - self.position.file
		} else {
			self.position.file
		};
		if (self.position.file + rank_dif) == other.position.file || (self.position.file - rank_dif) == other.position.file {
			return true;
		}
		if (self.position.rank + file_dif) == other.position.rank || (self.position.rank - file_dif) == other.position.rank {
			return true;
		}
		false
    }
}