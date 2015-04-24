enc16 <- as.numeric(scan("rsa_keygen/encryption_times16", what="int", sep="\n"))
enc32 <- as.numeric(scan("rsa_keygen/encryption_times32", what="int", sep="\n"))
enc64 <- as.numeric(scan("rsa_keygen/encryption_times64", what="int", sep="\n"))
enc96 <- as.numeric(scan("rsa_keygen/encryption_times96", what="int", sep="\n"))
enc128 <- as.numeric(scan("rsa_keygen/encryption_times128", what="int", sep="\n"))

dec16 <- as.numeric(scan("rsa_keygen/decryption_times16", what="int", sep="\n"))
dec32 <- as.numeric(scan("rsa_keygen/decryption_times32", what="int", sep="\n"))
dec64 <- as.numeric(scan("rsa_keygen/decryption_times64", what="int", sep="\n"))
dec96 <- as.numeric(scan("rsa_keygen/decryption_times96", what="int", sep="\n"))
dec128 <- as.numeric(scan("rsa_keygen/decryption_times128", what="int", sep="\n"))
means.enc = c(mean(enc16), mean(enc32), mean(enc64), mean(enc96), mean(enc128))
means.dec = c(mean(dec16), mean(dec32), mean(dec64), mean(dec96), mean(dec128))
bits = c(16,32,64,96,128)
png("cryptimes.png")
plot(bits, means.dec, col="red", xlab="Number of Bits",
		 ylab="Nanoseconds to Process")
title(main="Time to Encrypt vs. Time to Decrypt")
points(bits, means.enc, col="blue")
lines(bits, means.dec, col="red")
lines(bits, means.enc, col="blue")
legend(20,100000000, c("Decryption Time", "Encryption Time"), cex=0.8, 
			 col=c("red", "blue"), pch=1:2, lty=0:1)
dev.off()


#-------------------------------------------------------------------------------
# Modulo Exponentiation Stuff
#-------------------------------------------------------------------------------

schneier16 <- as.numeric(scan("rsa_keygen/schneier16", what="int", sep="\n"))
schneier18 <- as.numeric(scan("rsa_keygen/schneier18", what="int", sep="\n"))
schneier20 <- as.numeric(scan("rsa_keygen/schneier20", what="int", sep="\n"))
schneier22 <- as.numeric(scan("rsa_keygen/schneier22", what="int", sep="\n"))

naive16 <- as.numeric(scan("rsa_keygen/naive16", what="int", sep="\n"))
naive18 <- as.numeric(scan("rsa_keygen/naive18", what="int", sep="\n"))
naive20 <- as.numeric(scan("rsa_keygen/naive20", what="int", sep="\n"))
naive22 <- as.numeric(scan("rsa_keygen/naive22", what="int", sep="\n"))

means.schneier <- c(mean(schneier16), mean(schneier18), 
										mean(schneier20), mean(schneier22))
means.naive <- c(mean(naive16), mean(naive18), mean(naive20), mean(naive22))
numbits <- c(16,18,20,22)
png("modexp.png")
plot(numbits, means.naive, col="green",
		 xlab="Number of Bits", ylab="Nanoseconds to process")
title(main="Naive vs. Schneier's Modulo Exponentiation")
points(numbits, means.naive, col="green")
points(numbits, means.schneier, col="orange")
lines(numbits, means.naive, col="green")
lines(numbits, means.schneier, col="orange")
legend(16,10000000000, c("Naive Alg.", "Schneier's Alg"), cex=0.8, 
			 col=c("green", "orange"), pch=1:2, lty=0:1)
dev.off()
