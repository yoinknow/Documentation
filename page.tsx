"use client";

// Force dynamic rendering - prevents static generation errors
export const dynamic = 'force-dynamic';
export const revalidate = 0;

import { useState, useEffect } from "react";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import { usePrivy } from '@privy-io/react-auth';
import { User } from "lucide-react";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Zap, Coins, ArrowRight, Loader2, RefreshCw } from "lucide-react";
import { YoinkButton, YoinkButtonSecondary } from "@/components/YoinkButtons";
import { usePump } from "@/providers/PumpProvider";
import { useToast } from "@/components/ui/use-toast";
import { useRouter } from "next/navigation";
import { useTwitch } from "@/providers/TwitchProvider";
import { cn } from "@/lib/utils";
import SolanaIcon from "@/components/SolanaIcon";
import { Logo } from "@/components/Logo";
import { useGoogleAnalytics } from "@/providers/GoogleAnalyticsProvider";

export default function CreateStreamPage() {
  const { createCoin } = usePump();
  const { toast } = useToast();
  const router = useRouter();
  const { getCreatorStatusByUrl, loading: streamLoading } = useTwitch();
  const { trackEvent } = useGoogleAnalytics();

  // Set page metadata
  useEffect(() => {
    if (typeof document !== 'undefined') {
      document.title = "Launch Your Creator Token - Yoink";
    }
  }, []);

  const [streamTitle, setStreamTitle] = useState("");
  const [tokenName, setTokenName] = useState("");
  const [tokenSymbol, setTokenSymbol] = useState("");
  const [streamLink, setStreamLink] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [showSuccessLoader, setShowSuccessLoader] = useState(false);
  const [loadingStep, setLoadingStep] = useState(0); // For animated progress steps
  const [selectedImage, setSelectedImage] = useState<File | null>(null);
  const [imagePreview, setImagePreview] = useState<string | null>(null);
  const [streamPreview, setStreamPreview] = useState<string | null>(null);
  const [tokenDescription, setTokenDescription] = useState("");
  const [twitchUserId, setTwitchUserId] = useState<string | undefined>(undefined);
  const [twitchUserName, setTwitchUserName] = useState<string | undefined>(undefined);
  const [twitchProfilePicture, setTwitchProfilePicture] = useState<string | undefined>(undefined);
  const [showStream, setShowStream] = useState(false);
  const [showVideo, setShowVideo] = useState(false);
  const [selectedVideo, setSelectedVideo] = useState<File | null>(null);
  const [videoPreview, setVideoPreview] = useState<string | null>(null);
  const [uploadingVideo, setUploadingVideo] = useState(false);
  const [videoIpfsUrl, setVideoIpfsUrl] = useState<string | null>(null);
  const [showSocials, setShowSocials] = useState(false);
  const [showFirstBuy, setShowFirstBuy] = useState(false);
  const [buyAmount, setBuyAmount] = useState("");
  const [maxSolCost, setMaxSolCost] = useState("");
  const [isDragging, setIsDragging] = useState(false);
  const [socialLinks, setSocialLinks] = useState({
    website: "",
    twitter: "",
    telegram: ""
  });
  const [errors, setErrors] = useState({
    tokenName: "",
    tokenSymbol: "",
    streamTitle: "",
    image: "",
    description: "",
    video: "",
    website: "",
    twitter: "",
    telegram: "",
    buyAmount: "",
    maxSolCost: ""
  });

  const validateUrl = (url: string): boolean => {
    if (!url) return true; // Optional fields
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  };

  const handleImageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      if (file.size > 5 * 1024 * 1024) {
        setErrors(prev => ({ ...prev, image: "Image must be less than 5MB" }));
        return;
      }
      setSelectedImage(file);
      setImagePreview(URL.createObjectURL(file));
      setErrors(prev => ({ ...prev, image: "" }));
    }
  };

  const handleVideoChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      await processVideoFile(file);
    }
  };

  const processVideoFile = async (file: File) => {
    if (file.size > 100 * 1024 * 1024) {
      setErrors(prev => ({ ...prev, video: "Video must be less than 100MB" }));
      return;
    }
    if (!file.type.startsWith('video/')) {
      setErrors(prev => ({ ...prev, video: "Please select a valid video file" }));
      return;
    }
    
    setSelectedVideo(file);
    setVideoPreview(URL.createObjectURL(file));
    setErrors(prev => ({ ...prev, video: "" }));
    
    // Upload video to backend immediately
    await uploadVideoToPinata(file);
  };

  const handleDragOver = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  };

  const handleDrop = async (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    const file = e.dataTransfer.files?.[0];
    if (file) {
      await processVideoFile(file);
    }
  };

  const uploadVideoToPinata = async (file: File) => {
    setUploadingVideo(true);
    try {
      const formData = new FormData();
      formData.append('video', file);

      const response = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/upload-video`, {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        throw new Error('Failed to upload video');
      }

      const data = await response.json();
      setVideoIpfsUrl(data.ipfsUrl);
      
      toast({
        title: "Success",
        description: "Video uploaded successfully!",
      });
    } catch (error) {
      console.error('Error uploading video:', error);
      setErrors(prev => ({ ...prev, video: "Failed to upload video. Please try again." }));
      setSelectedVideo(null);
      setVideoPreview(null);
      toast({
        title: "Error",
        description: "Failed to upload video. Please try again.",
        variant: "destructive",
      });
    } finally {
      setUploadingVideo(false);
    }
  };

  // Format large numbers to K/M/B format
  const formatNumber = (num: number): string => {
    const absNum = Math.abs(num);
    if (absNum >= 1e9) return `${(num / 1e9).toFixed(2)}B`;
    if (absNum >= 1e6) return `${(num / 1e6).toFixed(2)}M`;
    if (absNum >= 1e3) return `${(num / 1e3).toFixed(2)}K`;
    return absNum < 1 ? num.toFixed(2) : num.toFixed(0);
  };

  // Calculate token amount ensuring total cost matches user's selected amount
  const calculateTokenAmount = (solAmount: number): { tokenAmount: number, maxCostLamports: number } => {
    if (!solAmount || solAmount <= 0) return { tokenAmount: 0, maxCostLamports: 0 };

    // Convert desired total SOL amount to lamports
    const desiredTotalLamports = Math.floor(solAmount * LAMPORTS_PER_SOL);

    // Work backwards to find base amount:
    // If total = base + (base * 0.0345), then base = total / (1 + 0.0345)
    const feeBasisPoints = 345; // 3.45% in basis points
    const baseCost = Math.floor(desiredTotalLamports * 10000 / (10000 + feeBasisPoints));

    // Initial reserves from contract
    const virtualSolReserves = 30000000001;  // Initial reserves in lamports
    const virtualTokenReserves = 1073000000000000;  // Initial reserves in raw units

    // Calculate token output using bonding curve formula with adjusted base amount
    const tokenAmount = Math.floor((baseCost * virtualTokenReserves) / virtualSolReserves);

    // For max cost, add 10% slippage to the total desired amount
    const maxCostLamports = Math.floor(desiredTotalLamports * 1.10);

    console.log('Buy calculation details:', {
      desiredTotalLamports,
      baseCost,
      tokenAmount,
      maxCostLamports,
      initialReserves: {
        sol: virtualSolReserves,
        token: virtualTokenReserves
      },
      formatted: {
        solAmount: solAmount.toFixed(9),
        tokenAmount: (tokenAmount / 1e6).toFixed(6),
        maxCostSol: (maxCostLamports / LAMPORTS_PER_SOL).toFixed(9)
      }
    });

    return { tokenAmount, maxCostLamports };
  };

  const validateForm = () => {
    const newErrors = {
      tokenName: "",
      tokenSymbol: "",
      streamTitle: "",
      image: "",
      description: "",
      video: "",
      website: "",
      twitter: "",
      telegram: "",
      buyAmount: "",
      maxSolCost: ""
    };

    // Validate URLs if provided
    if (socialLinks.website && !validateUrl(socialLinks.website)) {
      newErrors.website = "Please enter a valid URL";
    }
    if (socialLinks.twitter && !validateUrl(socialLinks.twitter)) {
      newErrors.twitter = "Please enter a valid URL";
    }
    if (socialLinks.telegram && !validateUrl(socialLinks.telegram)) {
      newErrors.telegram = "Please enter a valid URL";
    }
    if (!tokenName.trim()) {
      newErrors.tokenName = "Token name is required";
    } else if (tokenName.length > 32) {
      newErrors.tokenName = "Token name must be 32 characters or less";
    }
    
    if (!selectedImage) newErrors.image = "Please select an image";
    if (!tokenDescription.trim()) newErrors.description = "Token description is required";

    if (!tokenSymbol.trim()) {
      newErrors.tokenSymbol = "Token symbol is required";
    } else if (tokenSymbol.length > 10) {
      newErrors.tokenSymbol = "Token symbol must be 10 characters or less";
    } else if (!/^[A-Z0-9]+$/.test(tokenSymbol)) {
      newErrors.tokenSymbol = "Token symbol must contain only uppercase letters and numbers";
    }

    if (showStream && !streamTitle.trim()) {
      newErrors.streamTitle = "Stream title is required";
    }

    // Validate buy inputs if first buy is enabled
    if (showFirstBuy) {
      if (!buyAmount || parseFloat(buyAmount) <= 0) {
        newErrors.buyAmount = "Please enter a valid amount";
      }
      if (!maxSolCost || parseFloat(maxSolCost) <= 0) {
        newErrors.maxSolCost = "Please enter a valid maximum SOL amount";
      }
    }
    setErrors(newErrors);
    return !Object.values(newErrors).some(error => error !== "");
  };

  const { user: privyUser, login } = usePrivy();

  // Cycle through loading steps sequentially
  useEffect(() => {
    if (showSuccessLoader) {
      let currentStep = 0;
      let currentTimeout: NodeJS.Timeout;

      const showNextStep = () => {
        // If we've reached the last step (5), stay there
        if (currentStep >= 6) {
          return;
        }

        setLoadingStep(currentStep);
        currentStep++;

        // Continue to next step with a delay
        currentTimeout = setTimeout(showNextStep, 800);
      };

      // Start the cycle
      showNextStep();

      return () => clearTimeout(currentTimeout);
    }
  }, [showSuccessLoader]);

  // Loading steps configuration
  const loadingSteps = [
    { icon: Loader2, text: "Confirming transaction...", spin: true },
    { icon: RefreshCw, text: "Verifying token metadata...", spin: true },
    { icon: Coins, text: "Minting the SPL token", spin: false },
    { icon: Zap, text: "Setting up bonding curve...", spin: false },
    { icon: ArrowRight, text: "Updating all records", spin: false },
    { icon: Loader2, text: "Almost ready...", spin: true },
  ];

  return (
    <div className="container mx-auto px-2 sm:px-4 py-4 sm:py-8">
      {showSuccessLoader ? (
        <div className="fixed inset-0 bg-background/95 backdrop-blur-sm z-50 flex items-center justify-center">
          <div className="max-w-md w-full mx-4">
            <Card className="p-8 sm:p-10 shadow-2xl ">
              {/* Logo with Animation */}
              <div className="flex flex-col items-center space-y-8">
                <div className="relative">
                  {/* Animated rings */}
                  <div className="absolute inset-0 flex items-center justify-center">
                    <div className="w-32 h-32 sm:w-40 sm:h-40 rounded-full border-4 border-accent/20 border-t-accent animate-spin"></div>
                  </div>
                  <div className="absolute inset-0 flex items-center justify-center">
                    <div className="w-24 h-24 sm:w-32 sm:h-32 rounded-full border-4 border-accent/10 border-b-accent/50 animate-spin" style={{ animationDirection: 'reverse', animationDuration: '1.5s' }}></div>
                  </div>
                  
                  {/* Logo in center - FULL OPACITY */}
                  <div className="relative z-10 flex items-center justify-center w-32 h-32 sm:w-40 sm:h-40 opacity-100">
                    <Logo size="sm" />
                  </div>
                </div>
                
                {/* Status Text */}
                <div className="text-center space-y-3">
                  <h2 className="text-2xl sm:text-3xl font-bold text-foreground">
                    Creating your coin
                  </h2>
                  <p className="text-muted-foreground text-sm sm:text-base leading-relaxed">
                    Minting your token on the blockchain...
                  </p>
                </div>

                {/* Animated Progress Steps */}
                <div className="w-full space-y-3">
                  {loadingSteps.map((step, index) => {
                    const StepIcon = step.icon;
                    const isActive = index === loadingStep;
                    const isPast = index < loadingStep;
                    
                    return (
                      <div 
                        key={index}
                        className={cn(
                          "flex items-center gap-3 text-sm transition-all duration-500",
                          isActive ? "opacity-100 scale-100" : isPast ? "opacity-100" : "opacity-20"
                        )}
                      >
                        <div className={cn(
                          "flex-shrink-0 w-6 h-6 rounded-full flex items-center justify-center transition-all duration-500",
                          isActive ? "bg-accent/20 scale-110" : isPast ? "bg-accent/10" : "bg-secondary/30"
                        )}>
                          <StepIcon className={cn(
                            "h-4 w-4 transition-colors duration-500",
                            isActive ? "text-accent" : isPast ? "text-accent/60" : "text-muted-foreground",
                            step.spin && isActive && "animate-spin"
                          )} />
                        </div>
                        <span className={cn(
                          "transition-all duration-500",
                          isActive ? "text-foreground font-medium" : isPast ? "text-foreground/80" : "text-muted-foreground"
                        )}>
                          {step.text}
                        </span>
                      </div>
                    );
                  })}
                </div>

                {/* Bottom hint */}
                <div className="pt-4 border-t border-border/30 w-full">
                  <p className="text-xs text-center text-muted-foreground/70">
                    Please don't close this window
                  </p>
                </div>
              </div>
            </Card>
          </div>
        </div>
      ) : !privyUser ? (
        <div className="max-w-2xl mx-auto space-y-4 sm:space-y-6">
          <Card className="p-6 sm:p-8 shadow-sm">
            {/* Header Section */}
            <div className="flex flex-col items-center text-center space-y-6 mb-8">
              <div className="w-20 h-20 sm:w-24 sm:h-24 rounded-full bg-gradient-to-br from-accent/20 to-accent/10 border-2 border-accent/30 flex items-center justify-center shadow-lg">
                <Coins className="h-10 w-10 sm:h-12 sm:w-12 text-accent" />
              </div>
              
              <div className="space-y-2">
                <h2 className="text-2xl sm:text-3xl font-bold text-foreground">
                  Connect Your Wallet to Create
                </h2>
                <p className="text-base sm:text-lg text-muted-foreground max-w-md mx-auto leading-relaxed">
                  Bring your idea to life in seconds — mint your token directly on-chain with one click.
                </p>
              </div>
            </div>

            {/* Action Section */}
            <div className="flex flex-col items-center space-y-6">
              <YoinkButton 
                text="Connect Wallet" 
                onClick={login} 
                className="h-12 sm:h-14 px-8 sm:px-10 text-base sm:text-lg font-semibold min-w-[200px]"
              />
              
              {/* Info Box */}
              <div className="w-full p-4 rounded-lg bg-secondary/20 border border-border/30">
                <div className="flex items-start gap-3 text-sm text-muted-foreground">
                  <div className="flex-shrink-0 mt-0.5">
                    <svg className="h-5 w-5 text-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </div>
                  <div className="flex-1 leading-relaxed">
                    <span className="font-semibold text-foreground">First time in crypto?</span> Just login with your social network of choice and start creating.
                  </div>
                </div>
              </div>
            </div>
          </Card>

          {/* Feature Highlights */}
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <Card className="p-4 text-center">
              <div className="w-10 h-10 rounded-full bg-accent/10 flex items-center justify-center mx-auto mb-3">
                <Zap className="h-5 w-5 text-accent" />
              </div>
              <h3 className="font-semibold text-sm mb-1">Instant Launch</h3>

              <p className="text-xs text-muted-foreground">Deploy and trade your token instantly — no waiting, no coding.</p>
            </Card>
            
            <Card className="p-4 text-center">
              <div className="w-10 h-10 rounded-full bg-accent/10 flex items-center justify-center mx-auto mb-3">
                <Coins className="h-5 w-5 text-accent" />
              </div>
              <h3 className="font-semibold text-sm mb-1">Effortless Creation</h3>
              <p className="text-xs text-muted-foreground">No code, no hassle. Just connect, customize, and deploy.</p>
            </Card>
            
            <Card className="p-4 text-center">
              <div className="w-10 h-10 rounded-full bg-accent/10 flex items-center justify-center mx-auto mb-3">
                <ArrowRight className="h-5 w-5 text-accent" />
              </div>
              <h3 className="font-semibold text-sm mb-1">Lightning Fast</h3>
              <p className="text-xs text-muted-foreground">Go from idea to token in under 60 seconds.</p>
            </Card>
          </div>
        </div>
      ) : (
        <div className="max-w-2xl mx-auto space-y-4 sm:space-y-6">
          {/* Main Form */}
          <Card className="p-3 sm:p-6">
          <div className="space-y-4 sm:space-y-6">
            {/* Token Details */}
            <div>
              <h2 className="text-lg sm:text-xl font-semibold mb-4 sm:mb-6 text-foreground">Token Details</h2>
              <div className="flex flex-col sm:flex-row gap-4 sm:gap-6">
                {/* Image Upload */}
                <div className="flex-shrink-0 flex flex-col items-center sm:items-start">
                  <Label className="mb-1.5 block text-sm">Token Image</Label>
                  <div
                    className="h-20 w-20 sm:h-24 sm:w-24 bg-gradient-to-b from-secondary/40 to-secondary/20 rounded-full border border-border/20 shadow-sm cursor-pointer overflow-hidden relative hover:border-accent/50 hover:shadow-glow-sm transition-all duration-200"
                    onClick={() => document.getElementById('imageInput')?.click()}
                  >
                    <input
                      type="file"
                      id="imageInput"
                      className="hidden"
                      accept="image/*"
                      onChange={handleImageChange}
                    />
                    {imagePreview ? (
                      <img
                        src={imagePreview}
                        alt="Token Preview"
                        className="w-full h-full object-cover"
                      />
                    ) : (
                      <div className="absolute inset-0 flex items-center justify-center">
                        <div className="text-center">
                          <div className="text-[10px] sm:text-xs">Upload</div>
                          <div className="text-[10px] sm:text-xs text-muted-foreground">Max 5MB</div>
                        </div>
                      </div>
                    )}
                  </div>
                  {errors.image && (
                    <p className="text-xs text-accent mt-1 text-center sm:text-left">{errors.image}</p>
                  )}
                </div>

                {/* Token Info */}
                <div className="flex-grow space-y-3 sm:space-y-4">
                  <div>
                    <Label className="text-sm">Token Name</Label>
                    <Input
                      value={tokenName}
                      onChange={(e) => {
                        setTokenName(e.target.value);
                        if (errors.tokenName) setErrors(prev => ({ ...prev, tokenName: "" }));
                      }}
                      placeholder="e.g., Cyber Token, Neon Token"
                      className={cn("bg-secondary/20 border-border/20 text-sm h-9 sm:h-10", errors.tokenName && "border-accent/50")}
                      maxLength={32}
                    />
                    {errors.tokenName ? (
                      <p className="text-xs text-accent mt-1">{errors.tokenName}</p>
                    ) : (
                      <p className="text-xs text-muted-foreground mt-1">The full name of your token (max 32 characters)</p>
                    )}
                  </div>

                  <div>
                    <Label className="text-sm">Token Symbol</Label>
                    <Input
                      value={tokenSymbol}
                      onChange={(e) => {
                        const value = e.target.value.toUpperCase();
                        setTokenSymbol(value);
                        if (errors.tokenSymbol) setErrors(prev => ({ ...prev, tokenSymbol: "" }));
                      }}
                      placeholder="e.g., CYBER, NEON"
                      className={cn("bg-secondary/20 border-border/20 text-sm h-9 sm:h-10", errors.tokenSymbol && "border-accent/50")}
                      maxLength={10}
                    />
                    {errors.tokenSymbol ? (
                      <p className="text-xs text-accent mt-1">{errors.tokenSymbol}</p>
                    ) : (
                      <p className="text-xs text-muted-foreground mt-1">Trading symbol (max 10 characters)</p>
                    )}
                  </div>
                </div>
              </div>

              <div className="mt-3 sm:mt-4">
                <Label className="text-sm">Token Description</Label>
                <textarea
                  value={tokenDescription}
                  onChange={(e) => setTokenDescription(e.target.value)}
                  placeholder="Describe your token and its purpose..."
                  className={cn(
                    "w-full min-h-[60px] sm:min-h-[80px] bg-secondary/20 border-border/20 rounded-md p-2 text-sm resize-y",
                    errors.description && "border-accent/50"
                  )}
                  maxLength={500}
                />
                {errors.description ? (
                  <p className="text-xs text-accent mt-1">{errors.description}</p>
                ) : (
                  <p className="text-xs text-muted-foreground mt-1">A brief description of your token (max 500 characters)</p>
                )}
              </div>
            </div>

            {/* Social Links */}
            <div className="border-t border-border/20 pt-4 sm:pt-6">
              <div className="flex items-center justify-between gap-3 mb-4 sm:mb-6">
                <div className="flex flex-col gap-1 min-w-0 flex-1">
                  <Label htmlFor="socials-toggle" className="text-sm">Attach socials?</Label>
                </div>
                <Switch
                  id="socials-toggle"
                  checked={showSocials}
                  onCheckedChange={(checked) => {
                    setShowSocials(checked);
                    if (!checked) {
                      setSocialLinks({
                        website: "",
                        twitter: "",
                        telegram: ""
                      });
                    }
                  }}
                  className="transition-glow data-[state=checked]:bg-accent data-[state=checked]:shadow-glow data-[state=checked]:shadow-accent/40 shrink-0"
                />
              </div>

              {showSocials && (
                <div className="space-y-3 sm:space-y-4">
                  <div>
                    <Label className="text-sm">Website (Optional)</Label>
                    <Input
                      value={socialLinks.website}
                      onChange={(e) => {
                        setSocialLinks(prev => ({ ...prev, website: e.target.value }));
                        if (errors.website) setErrors(prev => ({ ...prev, website: "" }));
                      }}
                      placeholder="https://your-website.com"
                      className={cn("bg-secondary/20 border-border/20 text-sm h-9 sm:h-10", errors.website && "border-accent/50")}
                    />
                    {errors.website && (
                      <p className="text-xs text-accent mt-1">{errors.website}</p>
                    )}
                  </div>

                  <div>
                    <Label className="text-sm">Twitter/X (Optional)</Label>
                    <Input
                      value={socialLinks.twitter}
                      onChange={(e) => {
                        setSocialLinks(prev => ({ ...prev, twitter: e.target.value }));
                        if (errors.twitter) setErrors(prev => ({ ...prev, twitter: "" }));
                      }}
                      placeholder="https://twitter.com/yourusername"
                      className={cn("bg-secondary/20 border-border/20 text-sm h-9 sm:h-10", errors.twitter && "border-accent/50")}
                    />
                    {errors.twitter && (
                      <p className="text-xs text-accent mt-1">{errors.twitter}</p>
                    )}
                  </div>

                  <div>
                    <Label className="text-sm">Telegram (Optional)</Label>
                    <Input
                      value={socialLinks.telegram}
                      onChange={(e) => {
                        setSocialLinks(prev => ({ ...prev, telegram: e.target.value }));
                        if (errors.telegram) setErrors(prev => ({ ...prev, telegram: "" }));
                      }}
                      placeholder="https://t.me/yourusername"
                      className={cn("bg-secondary/20 border-border/20 text-sm h-9 sm:h-10", errors.telegram && "border-accent/50")}
                    />
                    {errors.telegram && (
                      <p className="text-xs text-accent mt-1">{errors.telegram}</p>
                    )}
                  </div>
                </div>
              )}
            </div>

            {/* First Buy Settings */}
            <div className="border-t border-border/20 pt-4 sm:pt-6">
              <div className="flex items-start justify-between gap-3 mb-4 sm:mb-6">
                <div className="flex flex-col gap-1 min-w-0 flex-1">
                  <Label htmlFor="first-buy-toggle" className="text-sm">Be the first buyer?</Label>
                  <p className="text-xs text-muted-foreground leading-relaxed">Helps preventing your coin from being sniped</p>
                </div>
                <Switch
                  id="first-buy-toggle"
                  checked={showFirstBuy}
                  onCheckedChange={(checked) => {
                    setShowFirstBuy(checked);
                    if (!checked) {
                      setBuyAmount("");
                      setMaxSolCost("");
                      setErrors(prev => ({ ...prev, buyAmount: "", maxSolCost: "" }));
                    }
                  }}
                  className="transition-glow data-[state=checked]:bg-accent data-[state=checked]:shadow-glow data-[state=checked]:shadow-accent/40 shrink-0 mt-0.5"
                />
              </div>

              {showFirstBuy && (
                <div className="space-y-3 sm:space-y-4">
                  <div>
                    <Label className="text-sm">Select Amount to Buy</Label>
                    <div className="grid grid-cols-2 sm:grid-cols-4 gap-2 mt-2">
                      {[0.5, 1, 1.5, 2].map((amount) => (
                      <YoinkButtonSecondary
                        key={amount}
                        text={<>
                          {amount} <SolanaIcon size="xs" className="inline-block ml-0.5" />
                        </>}
                        className={cn(
                          "w-full text-xs sm:text-sm h-8 sm:h-9",
                          parseFloat(buyAmount) === amount ?
                            "bg-gradient-to-b from-accent/30 to-accent/20 text-black border-accent/20 shadow-sm" :
                            "bg-secondary/20 border-border/20"
                        )}
                          onClick={() => {
                            setBuyAmount(amount.toString());
                            const { maxCostLamports } = calculateTokenAmount(amount);
                            setMaxSolCost((maxCostLamports / LAMPORTS_PER_SOL).toString());
                          }}
                        />
                      ))}
                    </div>
                  </div>

                  {buyAmount && (
                    <div className="bg-gradient-to-b from-secondary/30 to-secondary/20 p-3 sm:p-4 rounded-lg space-y-2 border border-border/20 shadow-sm">
                      <div className="flex justify-between text-xs sm:text-sm">
                        <span className="text-muted-foreground">Selected Amount:</span>
                        <span className="font-medium">{parseFloat(buyAmount).toFixed(3)} <SolanaIcon size="xs" className="inline-block ml-0.5" /></span>
                      </div>
                      <div className="flex justify-between text-xs sm:text-sm">
                        <span className="text-muted-foreground">Estimated Tokens:</span>
                        <span className="font-medium">
                          {buyAmount ? formatNumber(calculateTokenAmount(parseFloat(buyAmount)).tokenAmount / 1e6) : '0'} tokens
                        </span>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>

            {/* Stream Settings */}
            <div className="border-t border-border/20 pt-4 sm:pt-6">
              <div className="flex items-start justify-between gap-3 mb-4 sm:mb-6">
                <div className="flex flex-col gap-1 min-w-0 flex-1">
                  <Label htmlFor="stream-toggle" className="text-sm">Attach a Twitch stream?</Label>
                  <p className="text-xs text-muted-foreground leading-relaxed">If you do this, you won't be able to create your own stream later.</p>
                </div>
                <Switch
                  id="stream-toggle"
                  checked={showStream}
                  disabled={showVideo}
                  onCheckedChange={(checked) => {
                    setShowStream(checked);
                    if (!checked) {
                      setStreamTitle("");
                      setStreamLink("");
                      setStreamPreview(null);
                      setTwitchUserId(undefined);
                      setTwitchUserName(undefined);
                      setTwitchProfilePicture(undefined);
                    }
                  }}
                  className="transition-glow data-[state=checked]:bg-accent data-[state=checked]:shadow-glow data-[state=checked]:shadow-accent/40 shrink-0 mt-0.5"
                />
              </div>

              {showStream && (
                <div className="space-y-3 sm:space-y-4">
                  {/* Stream Preview */}
                  {streamPreview && (
                    <div className="aspect-video bg-gradient-to-br from-card/80 to-card/60 rounded-lg border border-border/20 shadow-sm relative overflow-hidden">
                      <img
                        src={streamPreview}
                        alt="Stream Preview"
                        className="w-full h-full object-cover"
                      />
                      <div className="absolute inset-0 bg-gradient-to-t from-background/60 to-transparent" />
                      <Badge className="absolute top-2 sm:top-3 left-2 sm:left-3 bg-gradient-to-b from-accent/90 to-accent/80 text-black border-accent/30 shadow-sm text-xs">
                        LIVE PREVIEW
                      </Badge>
                      {streamTitle && (
                        <div className="absolute bottom-2 sm:bottom-3 left-2 sm:left-3 right-2 sm:right-3">
                          <h3 className="text-sm sm:text-lg font-semibold text-foreground mb-1">
                            {streamTitle}
                          </h3>
                        </div>
                      )}
                    </div>
                  )}

                  <div>
                    <Label className="text-sm">Stream Title</Label>
                    <Input
                      value={streamTitle}
                      onChange={(e) => {
                        setStreamTitle(e.target.value);
                        if (errors.streamTitle) setErrors(prev => ({ ...prev, streamTitle: "" }));
                      }}
                      placeholder="Enter your stream title"
                      className={cn("bg-secondary/20 border-border/20 text-sm h-9 sm:h-10", errors.streamTitle && "border-accent/50")}
                    />
                    {errors.streamTitle && (
                      <p className="text-xs text-accent mt-1">{errors.streamTitle}</p>
                    )}
                  </div>

                  <div>
                    <Label className="text-sm">Stream Link (Optional)</Label>
                    <div className="flex gap-2">
                      <Input
                        value={streamLink}
                        onChange={(e) => setStreamLink(e.target.value)}
                        placeholder="e.g., https://twitch.tv/yourusername"
                        className="bg-secondary/20 border-border/20 text-sm h-9 sm:h-10"
                      />
                      <YoinkButtonSecondary
                        text=""
                        className="flex items-center justify-center w-8 sm:w-10 h-9 sm:h-10"
                        onClick={async () => {
                          if (!streamLink) return;
                          try {
                            const result = await getCreatorStatusByUrl(streamLink);
                            if (!result) {
                              toast({
                                title: "Error",
                                description: "Could not find stream information",
                                className: "bg-accent/10 border-accent/20 text-accent",
                              });
                              return;
                            }

                            if (result.status?.live) {
                              const stream = result.status.stream;
                              setStreamTitle(stream.title);
                              setTwitchUserId(stream.userId);
                              setTwitchUserName(stream.userName);
                              // Get profile picture from creator data
                              if (result.creator?.profileImageUrl) {
                                setTwitchProfilePicture(result.creator.profileImageUrl);
                              }
                              if (stream.thumbnailUrl) {
                                const thumbnailUrl = stream.thumbnailUrl.replace('{width}x{height}', '1920x1080');
                                setStreamPreview(thumbnailUrl);
                              }
                              toast({
                                title: "Success",
                                description: "Stream information loaded",
                              });
                            } else {
                              toast({
                                title: "Stream Offline",
                                description: "The stream is currently offline.",
                              });
                              setStreamPreview(null);
                            }
                          } catch (error) {
                            toast({
                              title: "Error",
                              description: "Failed to fetch stream information",
                              className: "bg-accent/10 border-accent/20 text-accent",
                            });
                          }
                        }}
                        disabled={!streamLink}
                        icon={<RefreshCw className={`h-3 w-3 sm:h-4 sm:w-4 ${streamLoading ? 'animate-spin' : ''}`} />}
                      />
                    </div>
                    <p className="text-xs text-muted-foreground mt-1">Your streaming platform URL</p>
                  </div>
                </div>
              )}
            </div>

            {/* Video Attachment Section */}
            <div className="border-t border-border/20 pt-4 sm:pt-6">
              <div className="flex items-start justify-between gap-3 mb-4 sm:mb-6">
                <div className="flex flex-col gap-1 min-w-0 flex-1">
                  <Label htmlFor="video-toggle" className="text-sm">Attach a Video?</Label>
                  <p className="text-xs text-muted-foreground leading-relaxed">Upload a video to showcase your coin (max 100MB).</p>
                </div>
                <Switch
                  id="video-toggle"
                  checked={showVideo}
                  disabled={showStream}
                  onCheckedChange={(checked) => {
                    setShowVideo(checked);
                    if (!checked) {
                      setSelectedVideo(null);
                      setVideoPreview(null);
                      setVideoIpfsUrl(null);
                      setErrors(prev => ({ ...prev, video: "" }));
                    }
                  }}
                  className="transition-glow data-[state=checked]:bg-accent data-[state=checked]:shadow-glow data-[state=checked]:shadow-accent/40 shrink-0 mt-0.5"
                />
              </div>

              {showVideo && (
                <div className="space-y-3 sm:space-y-4">
                  {/* Video Preview */}
                  {videoPreview && (
                    <div className="relative rounded-lg overflow-hidden border border-border/20 shadow-sm bg-gradient-to-br from-secondary/30 to-secondary/20">
                      <div className="aspect-video bg-black/40 relative overflow-hidden">
                        <video
                          src={videoPreview}
                          controls
                          className="w-full h-full object-contain"
                        />
                        <Badge className="absolute top-2 sm:top-3 left-2 sm:left-3 bg-gradient-to-b from-accent/90 to-accent/80 text-black border-accent/30 shadow-md text-xs">
                          VIDEO PREVIEW
                        </Badge>
                      </div>
                    </div>
                  )}

                  {/* Video Upload Area */}
                  <div className="relative">
                    {!videoPreview ? (
                      <div 
                        onClick={() => !uploadingVideo && document.getElementById('videoInput')?.click()}
                        onDragOver={handleDragOver}
                        onDragLeave={handleDragLeave}
                        onDrop={handleDrop}
                        className={cn(
                          "relative border-2 border-dashed rounded-lg p-6 sm:p-8 cursor-pointer transition-all duration-200",
                          uploadingVideo 
                            ? "border-accent/40 bg-accent/5 cursor-not-allowed" 
                            : isDragging
                              ? "border-accent bg-gradient-to-br from-accent/10 to-accent/5 shadow-glow scale-[1.02]"
                              : "border-border/30 bg-gradient-to-br from-secondary/20 to-secondary/10 hover:border-accent/50 hover:bg-gradient-to-br hover:from-accent/5 hover:to-accent/10 hover:shadow-glow-sm",
                          errors.video && "border-accent/60"
                        )}
                      >
                        <input
                          type="file"
                          id="videoInput"
                          className="hidden"
                          accept="video/*"
                          onChange={handleVideoChange}
                          disabled={uploadingVideo}
                        />
                        
                        <div className="flex flex-col items-center justify-center text-center space-y-3">
                          {uploadingVideo ? (
                            <>
                              <div className="w-12 h-12 sm:w-14 sm:h-14 rounded-full bg-gradient-to-br from-accent/20 to-accent/10 border-2 border-accent/30 flex items-center justify-center">
                                <Loader2 className="h-6 w-6 sm:h-7 sm:w-7 text-accent animate-spin" />
                              </div>
                              <div className="space-y-1">
                                <p className="text-sm font-medium text-foreground">Uploading to IPFS...</p>
                                <p className="text-xs text-muted-foreground">Please wait while we process your video</p>
                              </div>
                            </>
                          ) : (
                            <>
                              <div className={cn(
                                "w-12 h-12 sm:w-14 sm:h-14 rounded-full bg-gradient-to-br from-accent/20 to-accent/10 border-2 border-accent/30 flex items-center justify-center transition-all duration-200",
                                isDragging ? "scale-110" : "hover:scale-105"
                              )}>
                                <svg className="h-6 w-6 sm:h-7 sm:w-7 text-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                                </svg>
                              </div>
                              <div className="space-y-1">
                                <p className={cn(
                                  "text-sm font-medium transition-colors duration-200",
                                  isDragging ? "text-accent" : "text-foreground"
                                )}>
                                  {isDragging ? "Drop your video here" : "Click to upload or drag and drop"}
                                </p>
                                <p className="text-xs text-muted-foreground">
                                  MP4, WebM, AVI, MOV up to 100MB
                                </p>
                              </div>
                            </>
                          )}
                        </div>
                      </div>
                    ) : (
                      <div className="flex items-center justify-between p-3 sm:p-4 rounded-lg bg-gradient-to-br from-secondary/30 to-secondary/20 border border-border/20 shadow-sm">
                        <div className="flex items-center gap-3 min-w-0 flex-1">
                          <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-accent/20 to-accent/10 border border-accent/30 flex items-center justify-center flex-shrink-0">
                            <svg className="h-5 w-5 text-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
                            </svg>
                          </div>
                          <div className="min-w-0 flex-1">
                            <p className="text-sm font-medium text-foreground truncate">
                              {selectedVideo?.name || 'Video file'}
                            </p>
                            <p className="text-xs text-muted-foreground">
                              {selectedVideo && (selectedVideo.size / (1024 * 1024)).toFixed(2)} MB
                            </p>
                          </div>
                        </div>
                        <button
                          onClick={() => {
                            setSelectedVideo(null);
                            setVideoPreview(null);
                            setVideoIpfsUrl(null);
                            setErrors(prev => ({ ...prev, video: "" }));
                            const input = document.getElementById('videoInput') as HTMLInputElement;
                            if (input) input.value = '';
                          }}
                          className="h-8 w-8 rounded-md bg-red-500/10 hover:bg-red-500/20 border border-red-500/30 hover:border-red-500/50 flex items-center justify-center transition-all duration-200 flex-shrink-0"
                          title="Remove video"
                        >
                          <svg className="h-4 w-4 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                          </svg>
                        </button>
                      </div>
                    )}
                    
                    {/* Status Messages */}
                    {uploadingVideo && (
                      <div className="mt-2 p-2 rounded-md bg-accent/10 border border-accent/20">
                        <p className="text-xs text-accent flex items-center gap-2">
                          <Loader2 className="h-3 w-3 animate-spin" />
                          Uploading to IPFS... This may take a moment
                        </p>
                      </div>
                    )}
                    {errors.video && (
                      <div className="mt-2 p-2 rounded-md bg-accent/10 border border-accent/20">
                        <p className="text-xs text-accent">{errors.video}</p>
                      </div>
                    )}
                    {videoIpfsUrl && !uploadingVideo && !errors.video && (
                      <div className="mt-2 p-2 rounded-md bg-gradient-to-br from-green-500/10 to-green-500/5 border border-green-500/20">
                        <p className="text-xs text-green-500 flex items-center gap-2">
                          <svg className="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                          </svg>
                          Video uploaded successfully!
                        </p>
                      </div>
                    )}
                  </div>
                </div>
              )}
            </div>
          </div>
        </Card>

        {/* Token Preview */}
        <Card className="p-3 sm:p-4 shadow-sm space-y-3 sm:space-y-4">
          <div className="flex flex-col sm:flex-row items-center sm:items-start gap-3 sm:gap-4">
            {imagePreview ? (
              <div className="w-12 h-12 sm:w-16 sm:h-16 rounded-full border-2 border-accent/20 overflow-hidden flex items-center justify-center bg-gradient-to-b from-secondary/40 to-secondary/20 shadow-sm shrink-0">
                <img
                  src={imagePreview}
                  alt="Token Preview"
                  className="w-full h-full object-cover"
                />
              </div>
            ) : (
              <div className="w-12 h-12 sm:w-16 sm:h-16 rounded-full bg-gradient-to-b from-secondary/40 to-secondary/20 border-2 border-accent/20 shadow-sm flex items-center justify-center shrink-0">
                <Coins className="w-6 h-6 sm:w-8 sm:h-8 text-accent/50" />
              </div>
            )}
            <div className="text-center sm:text-left flex-1 min-w-0">
              <h3 className="text-base sm:text-lg font-semibold text-foreground truncate">
                {tokenName || "Your Token Name"}
                {tokenSymbol && (
                  <span className="ml-2 text-sm text-muted-foreground">
                    ${tokenSymbol}
                  </span>
                )}
              </h3>
              <p className="text-sm text-muted-foreground break-words">
                {tokenDescription || "Your token description will appear here"}
              </p>
            </div>
          </div>
        </Card>

        <div className="flex justify-center pt-2">
          <YoinkButton
            text={isCreating ? "Creating..." : "Create Coin"}
            onClick={async () => {
              if (!validateForm()) return;

              // Validate buy inputs if first buy is enabled
              if (showFirstBuy) {
                const newErrors = { ...errors };
                if (!buyAmount || parseFloat(buyAmount) <= 0) {
                  newErrors.buyAmount = "Please enter a valid amount";
                }
                if (!maxSolCost || parseFloat(maxSolCost) <= 0) {
                  newErrors.maxSolCost = "Please enter a valid maximum SOL amount";
                }
                setErrors(newErrors);
                if (newErrors.buyAmount || newErrors.maxSolCost) return;
              }
              try {
                setIsCreating(true);
                if (!createCoin) throw new Error("PumpProvider not properly initialized");
                if (!selectedImage) {
                  setErrors(prev => ({ ...prev, image: "Please select an image" }));
                  return;
                }

                const formData = new FormData();
                formData.append('file', selectedImage);
                const imageResponse = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/posts/upload-image/temp`, {
                  method: 'POST',
                  body: formData,
                });

                if (!imageResponse.ok) throw new Error('Failed to upload image');
                const { url: imageUrl } = await imageResponse.json();

                // Convert amounts to BN for createCoin
                // Token amount is already in raw units, maxSolCost is in lamports
                const { tokenAmount, maxCostLamports } = showFirstBuy ?
                  calculateTokenAmount(parseFloat(buyAmount)) :
                  { tokenAmount: 0, maxCostLamports: 0 };

                console.log('Creating coin with:', {
                  tokenAmount: tokenAmount.toString(),
                  maxCostLamports: maxCostLamports.toString(),
                  buyAmount,
                  maxSolCost
                });

                console.log('Creating coin with:', {
                  tokenAmount: tokenAmount.toString(),
                  maxCostLamports: maxCostLamports.toString(),
                  buyAmount,
                  maxSolCost
                });

                const result = await createCoin(
                  tokenName,
                  tokenSymbol,
                  imageUrl,
                  tokenDescription,
                  twitchUserName || tokenName,
                  streamLink,
                  new Date().toISOString(),
                  JSON.stringify({
                    title: streamTitle,
                    userId: twitchUserId,
                    thumbnailUrl: streamPreview,
                    streamerProfilePicture: twitchProfilePicture,
                    fullStatus: { live: true, stream: { thumbnailUrl: streamPreview } },
                    socialLinks,
                    videoLink: videoIpfsUrl || ''
                  }),
                  undefined,
                  showFirstBuy,
                  tokenAmount,
                  maxCostLamports,
                  twitchUserId,
                  showStream ? 'twitch' : 'none', // Only set platform to 'twitch' if a stream is attached
                  undefined
                );

                if (result.success) {
                  // Track successful token creation
                  trackEvent('Creator', 'Create Token', tokenSymbol, showFirstBuy ? parseFloat(buyAmount) : 0);

                  toast({
                    title: "Success!",
                    description: "Your token has been created successfully.",
                  });
                  
                  // Show loader for 5 seconds before redirecting
                  setShowSuccessLoader(true);
                  setIsCreating(false);
                  
                  setTimeout(() => {
                    if ((result as any).mintAddress) {
                      router.push(`/coin/${(result as any).mintAddress}`);
                    }
                  }, 5000);
                } else {
                  throw new Error((result as any).error || "Failed to create token");
                }
              } catch (error) {
                // Track token creation error
                trackEvent('Creator', 'Create Token Error', error instanceof Error ? error.message : 'Unknown', 0);

                toast({
                  title: "Error",
                  description: error instanceof Error ? error.message : "Failed to create token",
                  className: "bg-accent/10 border-accent/20 text-accent",
                });
              } finally {
                setIsCreating(false);
              }
            }}
            disabled={isCreating || !tokenName || !tokenSymbol || !tokenDescription || (showStream && !streamTitle)}
            className="h-10 sm:h-12 px-6 sm:px-8 text-sm sm:text-base min-w-[140px] sm:min-w-[160px]"
          />
        </div>
        </div>
      )}
    </div>
  );
}